use crate::dtos::redis_values::FileStatus;
use crate::dtos::token_claims::TokenClaims;
use crate::errors::Result;
use crate::services::file_handling::{get_container_client, upload_file_chunked, monitor_file_commit};
use crate::services::validate_field::validate_pdf_field;
use crate::state::app_state::FileHandlerState;
use axum::extract::{Multipart, State};
use axum::Json;

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
    let file = validate_pdf_field(field)?;
    let file_name = file.name.clone();
    let url = upload_file_chunked(file, &mut container_client).await?;
    let url = url.to_string();

    state.redis_client.set_ex(
        url.clone(),
        serde_json::to_string(&FileStatus { commited: false }).unwrap(),
        360,
    )?;

    monitor_file_commit(file_name, container_client, state.redis_client, url.clone(), 300);

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}
