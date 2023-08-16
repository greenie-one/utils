use crate::structs::token_claims::TokenClaims;
use crate::utils::validate_field::validate_image_field;
use crate::state::app_state::FileStorageState;
use crate::errors::api_errors::{APIResult, APIError};

use axum::extract::Multipart;
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn upload(
    State(mut state): State<FileStorageState>,
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> APIResult<Json<Value>> {
    let field = multipart.next_field().await?.ok_or_else(|| APIError::NoFileAttached)?;
    let file = validate_image_field(field, &user_details)?;
    let url = state.service.upload_file(file).await?;
    let url = url.to_string();

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}
