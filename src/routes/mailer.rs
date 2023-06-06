use std::env;

use axum::Router;

use crate::{handlers::mailer::mail_handler, utils::google_token::TokenHandler};

pub fn routes() -> Router {
    let mut key = env::current_dir().unwrap();
    key.push("./keys/local/googleapi/service-account-key.json");

    let state = TokenHandler::new(
        key.to_string_lossy().to_string(),
        "https://www.googleapis.com/auth/gmail.send".to_string(),
    )
    .unwrap();
    axum::Router::new()
        .route("/mailer/send_mail", axum::routing::post(mail_handler))
        .with_state(state)
}
