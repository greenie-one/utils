use axum::response::IntoResponse;

use crate::{
    database::user_documents::UserDocumentsCollection,
    env_config::APP_ENV,
    errors::api_errors::{APIError, APIResult},
    structs::{download_token::DownloadToken, files::File},
};

use super::file_storage::FileStorageService;

#[derive(Clone)]
pub struct DocDepotService {
    file_storage: FileStorageService,
}

impl DocDepotService {
    pub async fn new(user_container: String) -> APIResult<Self> {
        Ok(Self {
            file_storage: FileStorageService::new(user_container.as_str()),
        })
    }

    pub async fn create_container_if_not_exists(&self) -> APIResult<()> {
        if !self.file_storage.container_client.exists().await? {
            self.file_storage.container_client.create().await?;
        }
        Ok(())
    }

    pub fn from_token(token: String, user_container: String, filename: String) -> APIResult<Self> {
        let private_url = Self::constuct_url(user_container.clone(), filename);
        let token_url = DownloadToken::validate(token)?.url;
        if private_url != token_url {
            return Err(APIError::BadToken);
        }

        Ok(Self {
            file_storage: FileStorageService::new(user_container.as_str()),
        })
    }

    pub async fn doc_exists(
        &self,
        file_name: String,
        document_collection: UserDocumentsCollection,
    ) -> APIResult<bool> {
        let user_container = self.file_storage.container_client.container_name();
        let blob_client = self
            .file_storage
            .container_client
            .blob_client(file_name.clone());
        let url = Self::constuct_url(user_container.to_string(), file_name.to_string());
        if blob_client.exists().await? {
            let doc_exists = document_collection.exists(url).await?;
            if doc_exists {
                Err(APIError::FileAlreadyExists)?
            }
        }
        Ok(false)
    }

    pub async fn upload_file(&mut self, file: File<'_>, nonce: Vec<u8>) -> APIResult<String> {
        let url = self.file_storage.upload_file_encrypted(file, nonce).await?;

        let file_name = url.path_segments().unwrap().last().unwrap();
        let user_container = url.path_segments().unwrap().nth_back(1).unwrap();
        Ok(Self::constuct_url(
            user_container.to_string(),
            file_name.to_string(),
        ))
    }

    pub async fn download_file(
        &self,
        file_name: String,
        nonce: Vec<u8>,
    ) -> APIResult<impl IntoResponse> {
        self.file_storage
            .download_file_decrypted(file_name, nonce)
            .await
    }
}

impl DocDepotService {
    pub fn constuct_url(user_container: String, file_name: String) -> String {
        let env = APP_ENV.as_str();
        let url = match env {
            "production" => format!(
                "https://api.greenie.one/utils/doc_depot/{}/{}",
                user_container, file_name
            ),
            _ => format!(
                "https://dev-api.greenie.one/utils/doc_depot/{}/{}",
                user_container, file_name
            ),
        };
        url
    }
}
