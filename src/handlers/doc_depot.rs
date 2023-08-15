use crate::dtos::doc_depot::DownloadDTO;
use crate::errors::api_errors::APIResult;
use crate::services::doc_depot::DocDepotService;
use crate::utils::validate_field::validate_pdf_field;
use crate::{errors::api_errors::APIError, structs::token_claims::TokenClaims};
use axum::extract::{Multipart, Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};

pub async fn upload(user_details: TokenClaims, mut multipart: Multipart) -> APIResult<Json<Value>> {
    let mut service = DocDepotService::new(user_details.sub.clone());
    if !service.container_client.exists().await? {
        service.container_client.create().await?;
    }

    let field = multipart
        .next_field()
        .await?
        .ok_or_else(|| APIError::NoFileAttached)?;

    let file = validate_pdf_field(field)?;
    let url = service.upload_file(file).await?;

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}

pub async fn download(
    user_details: Option<TokenClaims>,
    Path((container_name, filename)): Path<(String, String)>,
    Query(query): Query<DownloadDTO>,
) -> APIResult<impl IntoResponse> {
    let service = match user_details {
        Some(user_details) => DocDepotService::new(user_details.sub),
        None => {
            let private_url = DocDepotService::constuct_url(container_name.clone(), filename.clone());
            let token = query
                .token
                .ok_or_else(|| APIError::MissingQueryParams("token".to_owned()))?;
            let token_url = DocDepotService::validate_token(token)?;
            println!("private_url: {}, token_url: {}", private_url, token_url);
            if private_url != token_url {
                return Err(APIError::BadToken);
            }
            DocDepotService::new(container_name.clone())
        }
    };

    let response = service.download_file(filename.to_owned()).await?;
    Ok(response)
}
