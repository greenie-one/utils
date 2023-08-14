use crate::{structs::token_claims::TokenClaims, errors::api_errors::APIError};
use crate::errors::api_errors::APIResult;
use crate::services::doc_depot::DocDepotService;
use crate::utils::validate_field::validate_pdf_field;
use axum::extract::{Multipart, Path};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::{json, Value};

pub async fn upload(
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> APIResult<Json<Value>> {
    let mut service = DocDepotService::new(user_details.sub.clone());
    if !service.container_client.exists().await? {
        service.container_client.create().await?;
    }

    let field = multipart.next_field().await?.ok_or_else(|| APIError::NoFileAttached)?;

    let file = validate_pdf_field(field)?;
    let url = service.upload_file(file).await?;
    let url = url.as_ref();

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}

pub async fn download(
    user_details: TokenClaims,
    Path(file_name): Path<String>,
) -> APIResult<impl IntoResponse> {
    let service = DocDepotService::new(user_details.sub.clone());

    let response = service.download_file(file_name).await?;

    Ok(response)
}
