use crate::env_config::{JWT_KEYS, APP_ENV};
use crate::errors::api_errors::{APIError, APIResult};
use crate::structs::download_token::DownloadToken;
use crate::structs::files::File;

use axum::http::header;
use axum::response::IntoResponse;
use azure_storage::StorageCredentials;
use azure_storage_blobs::blob::operations::GetBlobResponse;
use azure_storage_blobs::prelude::ClientBuilder;
use azure_storage_blobs::prelude::ContainerClient;
use futures_util::StreamExt;
use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct FileStorageService {
    pub container_client: ContainerClient,
}

impl FileStorageService {
    pub fn new(container_name: String) -> Self {
        Self {
            container_client: Self::get_container_client(container_name),
        }
    }

    pub fn from_token(token: String, container_name: String, filename: String) -> APIResult<Self> {
        let private_url =
            FileStorageService::constuct_url(container_name.clone(), filename);
        let token_url = FileStorageService::validate_token(token)?;
        if private_url != token_url {
            return Err(APIError::BadToken);
        }
        Ok(FileStorageService::new(container_name))
    }
}

impl FileStorageService {
    pub fn get_container_client(container_name: String) -> ContainerClient {
        let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
        let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");

        let storage_credentials = StorageCredentials::Key(account.clone(), access_key);

        ClientBuilder::new(account, storage_credentials).container_client(container_name)
    }

    pub fn constuct_url(container_name: String, file_name: String) -> String {
        let env = APP_ENV.as_str();
        let url = match env {
            "production" => format!(
                "https://api.greenie.one/utils/doc_depot/{}/{}",
                container_name, file_name
            ),
            _ => format!(
                "https://dev-api.greenie.one/utils/doc_depot/{}/{}",
                container_name, file_name
            ),
        };
        url
    }

    pub fn validate_token(token: String) -> APIResult<String> {
        let validation = Validation::new(Algorithm::RS256);
        let token_claims: TokenData<DownloadToken> =
            decode(token.as_ref(), &JWT_KEYS.decode_key, &validation)?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if token_claims.claims.exp < now {
            return Err(APIError::TokenExpired);
        }

        Ok(token_claims.claims.url)
    }
}

impl FileStorageService {
    pub async fn check_doc_exists(
        &self,
        file_name: String,
        document_collection: Collection<Document>,
    ) -> APIResult<bool> {
        let container_name = self.container_client.container_name();
        let blob_client = self.container_client.blob_client(file_name.clone());
        let url = Self::constuct_url(container_name.to_string(), file_name.to_string());
        if blob_client.exists().await? {
            let doc = document_collection
                .find_one(
                    doc! {
                        "privateUrl": url.clone()
                    },
                    None,
                )
                .await?;
            if doc.is_some() {
                return Err(APIError::FileAlreadyExists);
            }
        }
        Ok(false)
    }
}

impl FileStorageService {
    pub async fn upload_file<'a>(&mut self, file: File<'a>) -> APIResult<String> {
        let file_name = &file.name.clone();
        let content_type = &file.content_type.clone();

        let blob_client = self.container_client.blob_client(file_name);

        blob_client
            .put_block_blob(file.field.bytes().await?)
            .content_type(content_type)
            .await?;

        let container_name = self.container_client.container_name();
        let url = Self::constuct_url(container_name.to_string(), file_name.to_string());
        Ok(url)
    }

    pub async fn upload_file_encrypted<'a>(
        &mut self,
        file: File<'a>,
        nonce: Vec<u8>,
    ) -> APIResult<String> {
        let file_name = &file.name.clone();
        let content_type = &file.content_type.clone();

        let blob_client = self.container_client.blob_client(file_name);

        blob_client
            .put_block_blob(file.encrypt(nonce).await?)
            .content_type(content_type)
            .await?;

        let container_name = self.container_client.container_name();
        let url = Self::constuct_url(container_name.to_string(), file_name.to_string());
        Ok(url)
    }

    async fn fetch_file(&self, file_name: String) -> APIResult<GetBlobResponse> {
        let blob_client = self.container_client.blob_client(file_name);
        if blob_client.exists().await? == false {
            return Err(APIError::FileNotFound);
        }

        let mut data = blob_client.get().into_stream();
        let blob = data
            .next()
            .await
            .ok_or_else(|| APIError::InternalServerError("No data found".to_string()))??;

        Ok(blob)
    }

    // pub async fn download_file(&self, file_name: String) -> APIResult<impl IntoResponse> {
    //     let blob = self.fetch_file(file_name).await?;
    //     let content_type = blob.blob.properties.content_type;
    //     let file_name = blob.blob.name;
    //     let stream_body = StreamBody::new(blob.data);
    //     let headers = [
    //         (header::CONTENT_TYPE, content_type),
    //         (
    //             header::CONTENT_DISPOSITION,
    //             format!("form-data; name=\"file\"; filename=\"{}\"", file_name),
    //         ),
    //     ];
    //     Ok((headers, stream_body))
    // }

    pub async fn download_file_decrypted(
        &self,
        file_name: String,
        nonce: Vec<u8>,
    ) -> APIResult<impl IntoResponse> {
        let blob = self.fetch_file(file_name).await?;

        let content_type = blob.blob.properties.content_type;
        let file_name = blob.blob.name;
        let data = blob.data.collect().await?.to_vec();

        let body = File::decrypt(nonce, data)?;

        let headers = [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("form-data; name=\"file\"; filename=\"{}\"", file_name),
            ),
        ];
        Ok((headers, body))
    }
}
