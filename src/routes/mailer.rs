use std::path::PathBuf;

use axum::Router;

use crate::{handlers::mailer::mail_handler, state::google_token::TokenHandler};

pub fn routes() -> Router {
    let binding = PathBuf::from("./keys/local/googleapi/service-account-key.json");
    let x = binding.to_str().unwrap();
    let x = x.to_string();
    let state = TokenHandler::new(x.to_string(), "https://mail.google.com/".to_string()).unwrap();
    axum::Router::new()
        .route("/mailer/send_mail", axum::routing::post(mail_handler))
        .with_state(state)
}
