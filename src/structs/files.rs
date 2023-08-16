use axum::extract::multipart::Field;
use chacha20poly1305::{aead::Aead, XNonce};

use crate::{env_config::CIPHER, errors::api_errors::{APIError, APIResult}};

pub struct File<'a> {
    pub name: String,
    pub content_type: String,
    pub field: Field<'a>,
}

impl File<'_> {
    pub async fn encrypt(self, nonce: Vec<u8>) -> APIResult<Vec<u8>> {
        let nonce_parsed = XNonce::from_slice(&nonce);
        let bytes = self.field.bytes().await?;
        let cipher_text = CIPHER
            .encrypt(nonce_parsed, bytes.as_ref()).map_err(|_| APIError::InternalServerError("encryption failed".to_string()))?;
        Ok(cipher_text)
    }

    pub fn decrypt(nonce: Vec<u8>, data: Vec<u8>) -> APIResult<Vec<u8>> {
        let decrypted_file = CIPHER
        .decrypt(XNonce::from_slice(&nonce), data.as_ref()).map_err(|_| APIError::InternalServerError("encryption failed".to_string()))?;
        Ok(decrypted_file)
    }
}