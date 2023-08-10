use axum::{
    extract::multipart::MultipartError,
    http::header,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type APIResult<T> = std::result::Result<T, APIError>;
#[derive(Debug)]
pub enum APIError {
    Unauthorized,
    PayloadTooLarge,
    InvalidFileName,
    InvalidContentType,
    InavlidFileExtension,
    InternalServerError(String),
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

impl From<azure_core::Error> for APIError {
    fn from(value: azure_core::Error) -> Self {
        APIError::InternalServerError(format!("Azure Core Error: {:?}", value))
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

impl From<APIError> for ErrorMessages {
    fn from(value: APIError) -> Self {
        match value {
            APIError::Unauthorized => ErrorMessages {
                message: "Unauthorized".to_string(),
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                code: "GR0001",
            },
            APIError::InternalServerError(value) => ErrorMessages {
                message: value,
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "GR1002",
            },
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
        }
    }
}
