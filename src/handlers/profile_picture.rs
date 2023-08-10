use crate::dtos::token_claims::TokenClaims;
use crate::services::file_handling::upload_file;
use crate::services::validate_field::validate_image_field;
use crate::state::app_state::AppState;
use crate::errors::api_errors::APIResult;

use axum::extract::Multipart;
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn upload(
    State(mut state): State<AppState>,
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> APIResult<Json<Value>> {
    let field = multipart.next_field().await.unwrap().unwrap();
    let file = validate_image_field(field, &user_details)?;
    let url = upload_file(file, &mut state.container_client).await?;
    let url = url.to_string();

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}
