use crate::dtos::doc_depot::DownloadDTO;
use crate::errors::api_errors::APIResult;
use crate::models::user_nonces::UserNonce;
use crate::services::doc_depot::DocDepotService;
use crate::services::file_storage::{StorageEnum, FileStorageService};
use crate::state::app_state::DocDepotState;
use crate::structs::files::File;
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
    let mut service = FileStorageService::new(user_details.sub.clone(), StorageEnum::DocDepot);
    if !service.container_client.exists().await? {
        service.container_client.create().await?;
    }

    let field = multipart
        .next_field()
        .await?
        .ok_or_else(|| APIError::NoFileAttached)?;

    let file: File<'_> = File::try_from(field)?;
    file.validate_pdf()?;

    DocDepotService::check_doc_exists(
            &service.container_client,
            file.name.clone(),
            state.document_collection,
        )
        .await?;

    let user_nonce =
        UserNonce::create_or_fetch(user_details.sub.clone(), state.nonce_collection).await?;
    let url = service
        .upload_file_encrypted(file, user_nonce.nonce)
        .await?;

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
    println!("{:?}, {:?}", container_name, filename);
    let service = match user_details {
        Some(user_details) => FileStorageService::new(user_details.sub, StorageEnum::DocDepot),
        None => {
            let token = query
                .token
                .ok_or_else(|| APIError::MissingQueryParams("token".to_owned()))?;
            FileStorageService::from_token(token, container_name.clone(), filename.clone(), StorageEnum::DocDepot)?
        }
    };

    let user_nonce = UserNonce::fetch(container_name, state.nonce_collection).await?;
    let response = service
        .download_file_decrypted(filename.to_owned(), user_nonce.nonce)
        .await?;
    Ok(response)
}
