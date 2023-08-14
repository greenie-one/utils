use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};

use crate::errors::api_errors::APIResult;
use crate::errors::api_errors::APIError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub email: Option<String>,
    pub sub: String,
    pub iss: String,
    pub session_id: String,
    pub roles: Vec<String>,
    pub iat: u64,
    pub is_refresh: Option<bool>,
    pub exp: u64,
}

#[async_trait::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for TokenClaims {
    type Rejection = APIError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> APIResult<Self> {
        let token_str = parts
            .headers
            .get("x-user-details")
            .ok_or(APIError::Unauthorized)?
            .to_str()
            .unwrap();
        let token = serde_json::from_str::<TokenClaims>(token_str)?;
        Ok(token)
    }
}
