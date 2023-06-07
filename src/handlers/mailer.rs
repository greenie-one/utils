// use axum::extract::State;
use axum::Json;
use serde_json::{Value, json};
use crate::dtos::mailer::SendMailDto;
use crate::errors::Result;
// use crate::state::google_token::{TokenHandler};

pub async fn mail_handler(
    // State(state): State<TokenHandler>,
    Json(payload): Json<SendMailDto>,
) -> Result<Json<Value>> {
    println!("{:?}", payload);
    let _payload = &payload;

    // TODO: Implement mailer

    return Ok(Json(json!({
        "message": "Mail sent successfully"
    })));
}
