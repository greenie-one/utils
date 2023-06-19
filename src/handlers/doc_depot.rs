use crate::dtos::redis_values::FileStatus;
use crate::dtos::token_claims::TokenClaims;
use crate::env_config::FILE_VALIDATION_TIMEOUT;
use crate::errors::Result;
use crate::services::file_handling::{
    get_container_client, monitor_file_commit, upload_file_chunked,
};
use crate::services::validate_field::validate_pdf_field;
use crate::state::app_state::FileHandlerState;
use axum::extract::{Multipart, State};
use axum::Json;

use chrono::Utc;
use redis::Commands;
use serde_json::{json, Value};

pub async fn upload(
    State(mut state): State<FileHandlerState>,
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> Result<Json<Value>> {
    let mut container_client = get_container_client(user_details.sub.clone());
    if !container_client.exists().await? {
        container_client.create().await?;
    }

    let field = multipart.next_field().await.unwrap().unwrap();

    let mut file = validate_pdf_field(field)?;
    let url = upload_file_chunked(&mut file, &mut container_client).await?;
    let url = url.as_ref();

    state.redis_client.set_ex(
        url,
        serde_json::to_string(&FileStatus {
            commited: false,
            uplaod_time: Utc::now(),
        })
        .unwrap(),
        (*FILE_VALIDATION_TIMEOUT + 60) as usize,
    )?;

    monitor_file_commit(
        file.name,
        container_client,
        state.redis_client,
        url.to_string(),
        *FILE_VALIDATION_TIMEOUT,
    );

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}
