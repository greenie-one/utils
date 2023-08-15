use std::time::SystemTimeError;

use axum::{
    extract::multipart::MultipartError,
    http::header,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::log::error;

pub type APIResult<T> = std::result::Result<T, APIError>;
#[derive(Debug)]
pub enum APIError {
    Unauthorized,
    PayloadTooLarge,
    NoFileAttached,
    InvalidFileName,
    InvalidUrl,
    FileNotFound,
    FileAlreadyExists,
    InvalidContentType,
    InavlidFileExtension,
    InternalServerError(String),
    UserAlreadyExists,
    TokenExpired,
    InvalidToken,
    MissingQueryParams(String),
    BadToken,
}

impl From<jsonwebtoken::errors::Error> for APIError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        APIError::InvalidToken
    }
}
    
impl From<SystemTimeError> for APIError {
    fn from(value: SystemTimeError) -> Self {
        APIError::InternalServerError(format!("System Time Error: {:?}", value))
    }
}

impl From<bcrypt::BcryptError> for APIError {
    fn from(value: bcrypt::BcryptError) -> Self {
        APIError::InternalServerError(format!("Bcrypt Error: {:?}", value))
    }
}

impl From<url::ParseError> for APIError {
    fn from(_: url::ParseError) -> Self {
        APIError::InvalidUrl
    }
}

impl From<azure_core::Error> for APIError {
    fn from(value: azure_core::Error) -> Self {
        APIError::InternalServerError(format!("Azure Core Error: {:?}", value))
    }
}

impl From<mongodb::error::Error> for APIError {
    fn from(value: mongodb::error::Error) -> Self {
        APIError::InternalServerError(format!("MongoDB Error: {:?}", value))
    }
}
    
impl From<MultipartError> for APIError {
    fn from(value: MultipartError) -> Self {
        let status_code = value.status();
        match status_code {
            axum::http::StatusCode::PAYLOAD_TOO_LARGE => return APIError::PayloadTooLarge,
            _ => {
                return APIError::InternalServerError(format!(
                    "MultipartError: {:?}",
                    value.body_text()
                ))
            }
        }
    }
}

impl From<serde_json::Error> for APIError {
    fn from(value: serde_json::Error) -> Self {
        APIError::InternalServerError(format!("Serde Error: {:?}", value))
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let error_msg: ErrorMessages = self.into();
        error_msg.into_response()
    }
}

struct ErrorMessages {
    message: String,
    status_code: axum::http::StatusCode,
    code: &'static str,
}

impl ErrorMessages {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "message": self.message,
            "statusCode": self.status_code.as_str(),
            "code": self.code
        })
    }
}

impl From<APIError> for ErrorMessages {
    fn from(value: APIError) -> Self {
        match value {
            APIError::Unauthorized => ErrorMessages {
                message: "Unauthorized".to_string(),
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                code: "GR0001",
            },
            APIError::InternalServerError(err) => {
                error!("Internal Server Error: {}", err);
                return ErrorMessages {
                    message: "Internal server error".to_string(),
                    status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    code: "GR1002",
                }
            }
            APIError::PayloadTooLarge => ErrorMessages {
                message: "Payload too large".to_string(),
                status_code: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                code: "GR1003",
            },
            APIError::InvalidFileName => ErrorMessages {
                message: "File name error".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1004",
            },
            APIError::InvalidContentType => ErrorMessages {
                message: "Content type error".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1005",
            },
            APIError::InavlidFileExtension => ErrorMessages {
                message: "Invalid file extension".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1007",
            },
            APIError::NoFileAttached => ErrorMessages {
                message: "No file attached".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1008",
            },
            APIError::UserAlreadyExists => ErrorMessages {
                message: "User already exists".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GRA0003",
            },
            APIError::FileNotFound => ErrorMessages {
                message: "File not found".to_string(),
                status_code: axum::http::StatusCode::NOT_FOUND,
                code: "GR1009",
            },
            APIError::InvalidUrl => ErrorMessages {
                message: "Invalid url".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1010",
            },
            APIError::FileAlreadyExists => ErrorMessages {
                message: "File already exists".to_string(),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1011",
            },
            APIError::TokenExpired => ErrorMessages {
                message: "Download Token expired".to_string(),
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                code: "GR1012",
            },
            APIError::InvalidToken => ErrorMessages {
                message: "Invalid token".to_string(),
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                code: "GR1013",
            },
            APIError::MissingQueryParams(param) => ErrorMessages {
                message: format!("Missing query param: {}", param),
                status_code: axum::http::StatusCode::BAD_REQUEST,
                code: "GR1014",
            },
            APIError::BadToken => ErrorMessages {
                message: "Bad token".to_string(),
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                code: "GR1015",
            },
        }
    }
}

impl IntoResponse for ErrorMessages {
    fn into_response(self) -> Response {
        (
            self.status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(self.to_json()),
        )
            .into_response()
    }
}
