use axum::response::IntoResponse;

use crate::{env_config::APP_ENV, structs::{download_token::DownloadToken, files::File}, errors::api_errors::{APIError, APIResult}};

use super::file_storage::FileStorageService;

#[derive(Clone)]
pub struct LeadsService {
    pub file_storage: FileStorageService,
}

impl LeadsService {
    pub fn new() -> Self {
        Self {
            file_storage: FileStorageService::new("leads"),
        }
    }

    pub fn from_token(token: String, filename: String) -> APIResult<Self> {
        let private_url = Self::constuct_url(filename);
        let token_url = DownloadToken::validate(token)?.url;
        if private_url != token_url {
            return Err(APIError::BadToken);
        }

        Ok(Self {
            file_storage: FileStorageService::new("leads"),
        })
    }

    pub async fn upload_file(&mut self, file: File<'_>) -> APIResult<String> {
        let url = self.file_storage.upload_file(file).await?;
        Ok(url.to_string())
    }

    pub async fn download_file(&self, file_name: String) -> APIResult<impl IntoResponse> {
        self.file_storage.download_file(file_name).await
    }
}

impl LeadsService {
    pub fn constuct_url(file_name: String) -> String {
        let env = APP_ENV.as_str();
        let url = match env {
            "production" => format!("https://api.greenie.one/utils/leads/{}", file_name),
            _ => format!("https://dev-api.greenie.one/utils/leads/{}", file_name),
        };
        url
    }
}
