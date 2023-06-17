use crate::dtos::token_claims::TokenClaims;
use crate::errors::Result;
use crate::services::file_handling::{upload_file_chunked, get_container_client, get_container_sas};
use crate::services::validate_field::validate_pdf_field;
use axum::extract::Multipart;
use axum::{Json};

use serde_json::{json, Value};

pub async fn upload(
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> Result<Json<Value>> {
    let mut container_client = get_container_client(user_details.sub.clone());
    let sas = get_container_sas(&mut container_client, 10);
    println!("User details: {:?}", user_details);

    let field = multipart.next_field().await.unwrap().unwrap();
    let file = validate_pdf_field(field)?;
    let url = upload_file_chunked(file, container_client).await?;
    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}
