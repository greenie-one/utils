use axum::response::IntoResponse;

use crate::{
    database::user_documents::UserDocumentsCollection,
    env_config::APP_ENV,
    errors::api_errors::{APIError, APIResult},
    structs::{download_token::DownloadToken, files::File},
    utils::azure::get_container_client,
};

use super::FileStorageService;

#[derive(Clone)]
pub struct DocDepot();

impl DocDepot {
    pub async fn new(container_name: &str) -> APIResult<FileStorageService<Self>> {
        let container_client = get_container_client(container_name);
        if !container_client.exists().await? {
            container_client.create().await?;
        }
        Ok(FileStorageService {
            container_client,
            _phantom: std::marker::PhantomData::<DocDepot>,
        })
    }

    pub fn from_token(token: String, user_container: String, filename: String) -> APIResult<FileStorageService<Self>> {
        let private_url = Self::constuct_url(user_container.clone(), filename);
        let token_url = DownloadToken::validate(token)?.url;
        if private_url != token_url {
            return Err(APIError::BadToken);
        }

        Ok(FileStorageService {
            container_client: get_container_client(user_container.as_str()),
            _phantom: std::marker::PhantomData::<DocDepot>,
        })
    }

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

impl FileStorageService<DocDepot> {
    pub async fn doc_exists(
        &self,
        file_name: String,
        document_collection: UserDocumentsCollection,
    ) -> APIResult<bool> {
        let user_container = self.container_client.container_name();
        let blob_client = self.container_client.blob_client(file_name.clone());
        let url = DocDepot::constuct_url(user_container.to_string(), file_name.to_string());
        if blob_client.exists().await? {
            let doc_exists = document_collection.exists(url).await?;
            if doc_exists {
                Err(APIError::FileAlreadyExists)?
            }
        }
        Ok(false)
    }

    pub async fn upload_file(&mut self, file: File<'_>, nonce: Vec<u8>) -> APIResult<String> {
        let url = self.uploader_encrypted(file, nonce).await?;

        let file_name = url.path_segments().unwrap().last().unwrap();
        let user_container = url.path_segments().unwrap().nth_back(1).unwrap();
        Ok(DocDepot::constuct_url(
            user_container.to_string(),
            file_name.to_string(),
        ))
    }

    pub async fn download_file(
        &self,
        file_name: String,
        nonce: Vec<u8>,
    ) -> APIResult<impl IntoResponse> {
        self.downloader_decrypted(file_name, nonce).await
    }
}
