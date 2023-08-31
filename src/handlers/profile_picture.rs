use crate::errors::api_errors::{APIError, APIResult};
use crate::state::app_state::ProfilePicState;
use crate::structs::files::File;
use crate::structs::token_claims::TokenClaims;

use axum::extract::Multipart;
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn upload(
    State(mut state): State<ProfilePicState>,
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> APIResult<Json<Value>> {
    let field = multipart
        .next_field()
        .await?
        .ok_or_else(|| APIError::NoFileAttached)?;

    let mut file: File<'_> = File::try_from(field)?;
    file.validate_image()?;
    let file_extension = file.name.split('.').last().unwrap();
    file.name = format!("{}.{}", user_details.sub, file_extension);

    let url = state.service.upload_file(file).await?;
    let url = url.to_string();

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}
