use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UNAUTHORIZED,
    MailError,
    InternalServerError
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::UNAUTHORIZED => {
                (axum::http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response()
            }
            Error::MailError | Error::InternalServerError  => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR").into_response()
            }

        }
    }
}