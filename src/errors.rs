use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UNAUTHORIZED,
    MailError,
    GoogleAccessTokenError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::UNAUTHORIZED => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "UNAUTHORIZED").into_response()
            }
            Error::MailError => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "MailError").into_response()
            }
            Error::GoogleAccessTokenError => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "GoogleAccessTokenError")
                    .into_response()
            }
        }
    }
}