use crate::dtos::token_claims::TokenClaims;
use crate::errors::Result;
use crate::routes::profile_picture::AppState;
use crate::services::profile::{set_profile_picture, remove_profile_picture};
use crate::services::validate_field::validate_image_field;
use axum::extract::Multipart;
use axum::{extract::State, Json};

use serde_json::{json, Value};

pub async fn upload(
    State(state): State<AppState>,
    user_details: TokenClaims,
    mut multipart: Multipart,
) -> Result<Json<Value>> {
    println!("User details: {:?}", user_details);

    let field = multipart.next_field().await.unwrap().unwrap();
    let file = validate_image_field(field, &user_details)?;
    let url = set_profile_picture(user_details, state.db, file, state.container_client).await?;

    Ok(Json(json!({
        "message": "File uploaded successfully",
        "url": url
    })))
}

pub async fn delete(State(state): State<AppState>, user_details: TokenClaims) -> Result<Json<Value>> {
    println!("User details: {:?}", user_details);
    remove_profile_picture(user_details, state.db, state.container_client).await?;
    Ok(Json(json!({
        "message": "File deleted successfully"
    })))
}