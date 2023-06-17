use crate::dtos::{token_claims::TokenClaims, doc_depot::{CreateDocumentDto, UpdateDocumentDto}};
use crate::errors::Result;
use crate::services::file_handling::{upload_file_chunked, get_container_client, get_container_sas};
use crate::services::validate_field::validate_pdf_field;
use axum::extract::{Multipart};
use axum::{Json};

use serde_json::{json, Value};

pub async fn create(
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> Result<Json<Value>> {
    let field = multipart.next_field().await.unwrap().unwrap();
    let file = validate_pdf_field(field)?;
    let url = upload_file_chunked(file, get_container_client("documents".into())).await?;

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}

