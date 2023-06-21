use axum::{
    extract::multipart::MultipartError,
    response::{IntoResponse, Response},
};
use redis::RedisError;
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Unauthorized,

    PayloadTooLarge,
    InvalidFileName,
    InvalidContentType,
    InavlidFileExtension,

    InternalServerError(String),
}

impl From<MultipartError> for Error {
    fn from(value: MultipartError) -> Self {
        let status_code = value.status();
        match status_code {
            axum::http::StatusCode::PAYLOAD_TOO_LARGE => return Error::PayloadTooLarge,
            _ => {
                return Error::InternalServerError(format!(
                    "MultipartError: {:?}",
                    value.body_text()
                ))
            }
        }
    }
}

impl From<azure_core::Error> for Error {
    fn from(value: azure_core::Error) -> Self {
        Error::InternalServerError(format!("Azure Core Error: {:?}", value))
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        Error::InternalServerError(format!("Redis Error: {:?}", value))
    }
}
    
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Unauthorized => ErrorMessages {
                message: "Unauthorized".to_string(),
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                code: "GR0001",
            }
            .into_response(),
            Error::InternalServerError(value) => ErrorMessages {
                message: value,
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "GR1002",
            }
            .into_response(),
            Error::PayloadTooLarge => ErrorMessages {
                message: "Payload too large".to_string(),
                status_code: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                code: "GR1003",
            }
            .into_response(),
            Error::InvalidFileName => ErrorMessages {
                message: "File name error".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1004",
            }
            .into_response(),
            Error::InvalidContentType => ErrorMessages {
                message: "Content type error".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1005",
            }
            .into_response(),
            Error::InavlidFileExtension => ErrorMessages {
                message: "Invalid file extension".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1007",
            }
            .into_response(),
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
