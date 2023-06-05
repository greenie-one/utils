use axum::extract::State;
use axum::Json;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};
use serde_json::Value;

use crate::dtos::mailer::SendMailDto;
use crate::errors::Result;
use crate::utils::google_token::{TokenHandler};

pub async fn mail_handler(
    State(state): State<TokenHandler>,
    Json(payload): Json<SendMailDto>,
) -> Result<Json<Value>> {
    println!("{:?}", payload);
    let _payload = &payload;
    let email = Message::builder()
        .from("Greenie <office@greenie.one>".parse().unwrap())
        .to("Ratnesh <ratneshjain40@gmail.com>".parse().unwrap())
        .subject("Happy new year")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();
    let res = state.get_access_token().await;
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            println!("Error: {:?}", err);
            return Ok(Json(serde_json::json!({"message": "Email not sent"})));
        }
    };
    println!("{:?}", res);
    let creds = Credentials::new("office@greenie.one".to_owned(), res.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .authentication(vec![Mechanism::Xoauth2]
        )
        .build();

    match mailer.send(&email){
        Ok(_) => Ok(Json(
            serde_json::json!({"message": "Email sent successfully"}),
        )),
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(Json(serde_json::json!({"message": "Email not sent"})))
        }
    }
}
