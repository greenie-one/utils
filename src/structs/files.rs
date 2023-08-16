use axum::extract::multipart::Field;
use chacha20poly1305::{aead::Aead, Nonce};

use crate::{env_config::CIPHER, errors::api_errors::{APIError, APIResult}};

pub struct File<'a> {
    pub name: String,
    pub content_type: String,
    pub field: Field<'a>,
}

impl File<'_> {
    pub async fn encrypt(self, nonce: Vec<u8>) -> APIResult<Vec<u8>> {
        let bytes = self.field.bytes().await?;
        let cipher_text = CIPHER
            .encrypt(Nonce::from_slice(&nonce), bytes.as_ref()).map_err(|_| APIError::InternalServerError("encryption failed".to_string()))?;
        Ok(cipher_text)
    }
}