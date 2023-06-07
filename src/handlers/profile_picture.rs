use crate::{dtos::token_claims::TokenClaims, errors::Result};
use axum::extract::Multipart;
use axum::http::HeaderMap;
use axum::{extract::State, Json, TypedHeader};
use mongodb::Client;
use serde_json::{json, Value};

pub async fn upload(
    State(state): State<Client>,
    user_details: HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<Value>> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
    Ok(Json(json!({
        "message": "File uploaded successfully"
    })))
}
