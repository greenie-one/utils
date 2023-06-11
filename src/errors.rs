use axum::{response::{IntoResponse, Response}, extract::multipart::MultipartError};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Unauthorized,
    MailError,
    InternalServerError(String),
}

impl From<MultipartError> for Error {
    fn from(value: MultipartError) -> Self {
        Error::InternalServerError(format!("MultipartError: {:?}", value))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Unauthorized => ErrorMessages {
                message: "Unauthorized".to_string(),
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                code: "GR001",
            }
            .into_response(),
            Error::MailError => ErrorMessages {
                message: "Mail Error".to_string(),
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "GR102",
            }
            .into_response(),
            Error::InternalServerError(value) => ErrorMessages {
                message: value,
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "GR103",
            }.into_response(),
        }
    }
}

struct ErrorMessages {
    message: String,
    status_code: axum::http::StatusCode,
    code: &'static str,
}

impl ToString for ErrorMessages {
    fn to_string(&self) -> String {
        json!({
            "message": self.message,
            "statusCode": self.status_code.as_str(),
            "code": self.code
        })
        .to_string()
    }
}

impl IntoResponse for ErrorMessages {
    fn into_response(self) -> Response {
        (self.status_code, self.to_string()).into_response()
    }
}
