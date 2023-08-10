use crate::dtos::token_claims::TokenClaims;
use crate::errors::api_errors::APIResult;
use crate::services::file_handling::{
    get_container_client, upload_file,
};
use crate::services::validate_field::validate_pdf_field;
use axum::extract::Multipart;
use axum::Json;
use serde_json::{json, Value};

pub async fn upload(
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> APIResult<Json<Value>> {
    let mut container_client = get_container_client(user_details.sub.clone());
    if !container_client.exists().await? {
        container_client.create().await?;
    }

    let field = multipart.next_field().await.unwrap().unwrap();

    let file = validate_pdf_field(field)?;
    let url = upload_file(file, &mut container_client).await?;
    let url = url.as_ref();

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}
