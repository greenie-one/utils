use axum::Router;

use crate::{handlers::mailer::mail_handler, utils::google_token::TokenHandler};

pub fn routes() -> Router {
    let state = TokenHandler::new("D:\\Projects\\Work\\Greenie\\utils\\keys\\local\\googleapi\\service-account-key.json".to_string(), "https://mail.google.com/".to_string()).unwrap();
    axum::Router::new()
        .route("/mailer/send_mail", axum::routing::post(mail_handler)).with_state(state)
}