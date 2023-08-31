use axum::extract::multipart::Field;
use chacha20poly1305::{aead::Aead, XNonce};

use crate::{
    env_config::CIPHER,
    errors::api_errors::{APIError, APIResult},
};

pub struct File<'a> {
    pub name: String,
    pub content_type: String,
    pub field: Field<'a>,
}

impl File<'_> {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl File<'_> {
    pub fn validate_pdf(&self) -> APIResult<()> {
        if !self.content_type.starts_with("application/pdf") {
            return Err(APIError::InvalidContentType);
        }
        if !self.name.ends_with(".pdf") {
            return Err(APIError::InavlidFileExtension);
        }
        Ok(())
    }

    pub fn validate_image(&self) -> APIResult<()> {
        if !self.content_type.starts_with("image/") {
            return Err(APIError::InvalidContentType);
        }
        if !self.name.ends_with(".jpg")
            && !self.name.ends_with(".jpeg")
            && !self.name.ends_with(".png")
        {
            return Err(APIError::InavlidFileExtension);
        }
        Ok(())
    }

    pub fn validate_csv(&self) -> APIResult<()> {
        if !self.content_type.starts_with("text/csv")
            && !self.content_type.starts_with("application/vnd.ms-excel")
            && !self
                .content_type
                .starts_with("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        {
            return Err(APIError::InvalidContentType);
        }
        if !self.name.ends_with(".csv")
            && !self.name.ends_with(".xls")
            && !self.name.ends_with(".xlsx")
        {
            return Err(APIError::InavlidFileExtension);
        }
        Ok(())
    }
}

impl File<'_> {
    pub async fn encrypt(self, nonce: Vec<u8>) -> APIResult<Vec<u8>> {
        let nonce_parsed = XNonce::from_slice(&nonce);
        let bytes = self.field.bytes().await?;
        let cipher_text = CIPHER
            .encrypt(nonce_parsed, bytes.as_ref())
            .map_err(|_| APIError::InternalServerError("encryption failed".to_string()))?;
        Ok(cipher_text)
    }

    pub fn decrypt(nonce: Vec<u8>, data: Vec<u8>) -> APIResult<Vec<u8>> {
        let decrypted_file = CIPHER
            .decrypt(XNonce::from_slice(&nonce), data.as_ref())
            .map_err(|_| APIError::InternalServerError("encryption failed".to_string()))?;
        Ok(decrypted_file)
    }
}

impl<'a> TryFrom<Field<'a>> for File<'a> {
    type Error = APIError;

    fn try_from(field: Field) -> APIResult<File<'_>> {
        let file_name = field.file_name().ok_or_else(|| APIError::InvalidFileName)?;
        let content_type = field
            .content_type()
            .ok_or_else(|| APIError::InvalidContentType)?;

        Ok(File {
            name: file_name.to_string(),
            content_type: content_type.to_string(),
            field,
        })
    }
}
