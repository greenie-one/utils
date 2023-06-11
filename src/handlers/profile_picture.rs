use crate::dtos::token_claims::TokenClaims;
use crate::errors::Result;
use crate::routes::profile_picture::AppState;
use crate::services::file_handling::{upload_file_chunked};
use axum::extract::Multipart;
use axum::{extract::State, Json};

use serde_json::{json, Value};

pub async fn upload(
    State(state): State<AppState>,
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> Result<Json<Value>> {
    println!("User details: {:?}", user_details);

    while let Some(field) = multipart.next_field().await.unwrap() {
        upload_file_chunked(field, state.container_client.clone()).await?;
    }
    Ok(Json(json!({
        "message": "File uploaded successfully"
    })))
}
