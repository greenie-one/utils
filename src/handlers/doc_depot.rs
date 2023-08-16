use crate::dtos::doc_depot::DownloadDTO;
use crate::errors::api_errors::APIResult;
use crate::models::user_nonces::UserNonce;
use crate::services::file_storage::FileStorageService;
use crate::state::app_state::DocDepotState;
use crate::utils::validate_field::validate_pdf_field;
use crate::{errors::api_errors::APIError, structs::token_claims::TokenClaims};
use axum::extract::{Multipart, Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};

pub async fn upload(
    State(state): State<DocDepotState>,
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> APIResult<Json<Value>> {
    let mut service = FileStorageService::new(user_details.sub.clone());
    if !service.container_client.exists().await? {
        service.container_client.create().await?;
    }

    let field = multipart
        .next_field()
        .await?
        .ok_or_else(|| APIError::NoFileAttached)?;

    let file = validate_pdf_field(field)?;
    service.file_exists(file.name.clone(), state.document_collection).await?;

    let user_nonce = UserNonce::create_or_fetch(user_details.sub.clone(), state.nonce_collection).await?;
    let url = service.upload_file_encrypted(file, user_nonce.nonce).await?;

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}

pub async fn download(
    State(state): State<DocDepotState>,
    user_details: Option<TokenClaims>,
    Path((container_name, filename)): Path<(String, String)>,
    Query(query): Query<DownloadDTO>,
) -> APIResult<impl IntoResponse> {
    let service = match user_details {
        Some(user_details) => FileStorageService::new(user_details.sub),
        None => {
            let private_url =
                FileStorageService::constuct_url(container_name.clone(), filename.clone());
            let token = query
                .token
                .ok_or_else(|| APIError::MissingQueryParams("token".to_owned()))?;
            let token_url = FileStorageService::validate_token(token)?;
            if private_url != token_url {
                return Err(APIError::BadToken);
            }
            FileStorageService::new(container_name.clone())
        }
    };

    let user_nonce = UserNonce::fetch(container_name, state.nonce_collection).await?;
    let response = service.download_file_decrypted(filename.to_owned(), user_nonce.nonce).await?;
    Ok(response)
}
