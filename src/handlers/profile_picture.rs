use mongodb::{Client};
use axum::{Json, extract::State, TypedHeader};
use serde_json::{Value, json};
use crate::{errors::Result, dtos::token_claims::TokenClaims};
use axum::extract::Multipart;

pub async fn upload(mut multipart: Multipart, TypedHeader(user_details): TypedHeader<TokenClaims>, State(state): State<Client>)  -> Result<Json<Value>> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
    Ok(Json(json!({
        "message": "File uploaded successfully"
    })))
}