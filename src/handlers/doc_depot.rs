use crate::dtos::doc_depot::DownloadDTO;
use crate::errors::api_errors::APIResult;
use crate::services::file_storage::doc_depot::DocDepot;
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
    let field = multipart.next_field().await?;
    let field = match field {
        Some(field) => field,
        None => return Err(APIError::NoFileAttached),
    };

    let file: File<'_> = File::try_from(field)?;
    file.validate_pdf()?;

    let mut service = DocDepot::new(user_details.sub.as_str()).await?;
    service
        .doc_exists(file.name.as_ref(), state.document_collection)
        .await?;

    let user_nonce = state
        .nonce_collection
        .create_or_fetch(user_details.sub.as_ref())
        .await?;

    let url = service.upload_file(file, user_nonce.nonce).await?;

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}

pub async fn download(
    State(state): State<DocDepotState>,
    user_details: Option<TokenClaims>,
    Path((user_container, filename)): Path<(String, String)>,
    Query(query): Query<DownloadDTO>,
) -> APIResult<impl IntoResponse> {
    let service = match user_details {
        Some(user_details) => DocDepot::new(user_details.sub.as_str()).await?,
        None => {
            let token = query
                .token
                .ok_or_else(|| APIError::MissingQueryParams("token".to_owned()))?;
            DocDepot::from_token(token, user_container.as_ref(), filename.as_ref())?
        }
    };

    let user_nonce = state.nonce_collection.fetch(user_container).await?;
    let response = service
        .download_file(filename.to_owned(), user_nonce.nonce)
        .await?;
    Ok(response)
}
