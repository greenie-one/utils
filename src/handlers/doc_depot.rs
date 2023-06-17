use crate::dtos::token_claims::TokenClaims;
use crate::errors::Result;
use crate::state::app_state::AppState;
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

    Ok(Json(json!({
        "message": "File uploaded successfully",
    })))
}

pub async fn delete(State(state): State<AppState>, user_details: TokenClaims) -> Result<Json<Value>> {
    println!("User details: {:?}", user_details);
    Ok(Json(json!({
        "message": "File deleted successfully"
    })))
}