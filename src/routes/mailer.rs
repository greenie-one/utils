use axum::Router;

use crate::{handlers::mailer::mail_handler, utils::google_token::TokenHandler};

pub fn routes() -> Router {
    let state = TokenHandler::new();
    axum::Router::new()
        .route("/mailer/send_mail", axum::routing::post(mail_handler)).with_state(state)
}