use crate::{structs::token_claims::TokenClaims, errors::api_errors::APIError};
use crate::errors::api_errors::APIResult;
use crate::services::doc_depot::DocDepotService;
use crate::utils::validate_field::validate_pdf_field;
use axum::extract::Multipart;
use axum::Json;
use serde_json::{json, Value};

pub async fn upload(
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> APIResult<Json<Value>> {
    let mut doc_depot_client = DocDepotService::new(user_details.sub.clone());
    if !doc_depot_client.container_client.exists().await? {
        doc_depot_client.container_client.create().await?;
    }

    let field = multipart.next_field().await?.ok_or_else(|| APIError::NoFileAttached)?;

    let file = validate_pdf_field(field)?;
    let url = doc_depot_client.upload_file(file).await?;
    let url = url.as_ref();

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}
