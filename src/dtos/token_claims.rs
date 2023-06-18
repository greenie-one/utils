use crate::Result;
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};

use crate::Error;

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
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let token_str = parts
            .headers
            .get("x-user-details")
            .ok_or(Error::Unauthorized)?
            .to_str()
            .unwrap();
        let token = serde_json::from_str::<TokenClaims>(token_str);
        match token {
            Ok(token) => Ok(token),
            Err(_) => Err(Error::InternalServerError(
                "Invalid token - cannot parse".to_string(),
            )),
        }
    }
}
