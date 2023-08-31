use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{env_config::JWT_KEYS, errors::api_errors::{APIResult, APIError}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadToken {
    pub url: String,
    pub iat: u64,
    pub exp: u64,
}

impl DownloadToken {
    pub fn new_from_days(url: String, days: u64) -> APIResult<Self> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let expiry = now + std::time::Duration::from_secs(60 * 60 * 24 * days).as_secs();
        Ok(Self {
            url,
            iat: now,
            exp: expiry,
        })
    }

    pub fn encode(&self) -> String {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
            self,
            &JWT_KEYS.encode_key,
        )
        .unwrap();
        token
    }

    pub fn validate(token: String) -> APIResult<Self> {
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        let token_claims: jsonwebtoken::TokenData<DownloadToken> =
            jsonwebtoken::decode(token.as_ref(), &JWT_KEYS.decode_key, &validation)?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if token_claims.claims.exp < now {
            return Err(APIError::TokenExpired);
        }

        Ok(token_claims.claims)
    }
}
