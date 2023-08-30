use crate::env_config::{JWT_KEYS, STORAGE_ACCOUNT, STORAGE_ACCESS_KEY};
use crate::errors::api_errors::{APIError, APIResult};
use crate::structs::download_token::DownloadToken;
use crate::structs::files::File;

use axum::body::StreamBody;
use axum::http::header;
use axum::response::IntoResponse;
use azure_storage::StorageCredentials;
use azure_storage_blobs::blob::operations::GetBlobResponse;
use azure_storage_blobs::prelude::ClientBuilder;
use azure_storage_blobs::prelude::ContainerClient;
use futures_util::StreamExt;
use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

use super::doc_depot::DocDepotService;
use super::leads::LeadsService;
use super::profile::ProfileService;

#[derive(Clone)]
pub enum StorageEnum {
    DocDepot,
    Leads,
    ProfilePicture,
}

impl StorageEnum {
    pub fn constuct_url(&self, container_name: String, file_name: String) -> String {
        match self {
            Self::DocDepot => DocDepotService::constuct_url(container_name, file_name),
            Self::Leads => LeadsService::constuct_url(container_name, file_name),
            Self::ProfilePicture => ProfileService::constuct_url(container_name, file_name),
        }
    }
}

#[derive(Clone)]
pub struct FileStorageService {
    pub container_client: ContainerClient,
    pub storage_service: StorageEnum,
}

impl FileStorageService {
    pub fn new(container_name: String, service: StorageEnum) -> Self {
        Self {
            container_client: Self::get_container_client(container_name),
            storage_service: service,
        }
    }

    pub fn from_token(
        token: String,
        container_name: String,
        filename: String,
        service: StorageEnum,
    ) -> APIResult<Self> {
        let private_url = service.constuct_url(container_name.clone(), filename);
        let token_url = FileStorageService::validate_token(token)?;
        if private_url != token_url {
            return Err(APIError::BadToken);
        }
        Ok(Self {
            container_client: Self::get_container_client(container_name),
            storage_service: service,
        })
    }
}

impl FileStorageService {
    fn get_container_client(container_name: String) -> ContainerClient {
        let storage_credentials = StorageCredentials::Key(STORAGE_ACCOUNT.clone(), STORAGE_ACCESS_KEY.clone());
        ClientBuilder::new(STORAGE_ACCOUNT.clone(), storage_credentials).container_client(container_name)
    }

    fn validate_token(token: String) -> APIResult<String> {
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
    pub async fn upload_file<'a>(&mut self, file: File<'a>) -> APIResult<String> {
        let file_name = &file.name.clone();
        let content_type = &file.content_type.clone();

        let blob_client = self.container_client.blob_client(file_name);

        blob_client
            .put_block_blob(file.field.bytes().await?)
            .content_type(content_type)
            .await?;

        let container_name = self.container_client.container_name();
        let url = self
            .storage_service
            .constuct_url(container_name.to_string(), file_name.to_string());
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
        let url = self
            .storage_service
            .constuct_url(container_name.to_string(), file_name.to_string());
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

    pub async fn download_file(&self, file_name: String) -> APIResult<impl IntoResponse> {
        let blob = self.fetch_file(file_name).await?;
        let content_type = blob.blob.properties.content_type;
        let file_name = blob.blob.name;
        let stream_body = StreamBody::new(blob.data);
        let headers = [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("form-data; name=\"file\"; filename=\"{}\"", file_name),
            ),
        ];
        Ok((headers, stream_body))
    }

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
