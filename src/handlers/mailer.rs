use axum::extract::State;
use axum::Json;
use mail_send::mail_builder::MessageBuilder;
use mail_send::{Credentials, SmtpClientBuilder};
use serde_json::Value;

use crate::dtos::mailer::SendMailDto;
use crate::errors::Result;
use crate::utils::google_token::{TokenHandler, test};

pub async fn mail_handler(
    State(state): State<TokenHandler>,
    Json(payload): Json<SendMailDto>,
) -> Result<Json<Value>> {
    println!("{:?}", payload);
    let _payload = &payload;
    let message = MessageBuilder::new()
        .from(("John Doe", "john@example.com"))
        .to(vec![
            ("Jane Doe", "jane@example.com"),
            ("James Smith", "james@test.com"),
        ])
        .subject("Hi!")
        .html_body("<h1>Hello, world!</h1>")
        .text_body("Hello world!");

    let res = state.get_access_token().await?;
    println!("Res {}\n", res);

    test().await;
    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    let mut sender = SmtpClientBuilder::new("smtp.gmail.com", 587)
        .implicit_tls(false)
        .credentials(Credentials::XOauth2 { username: ("office@greenie.one"), secret: (res.as_str()) })
        .connect()
        .await
        .unwrap();

    match sender.send(message).await {
        Ok(_) => Ok(Json(
            serde_json::json!({"message": "Email sent successfully"}),
        )),
        Err(_) => Ok(Json(serde_json::json!({"message": "Email not sent"}))),
    }
}
