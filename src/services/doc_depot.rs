use std::time::{UNIX_EPOCH, SystemTime};

use crate::env_config::DECODE_KEY;
use crate::errors::api_errors::{APIError, APIResult};
use crate::structs::download_token::DownloadToken;
use axum::body::StreamBody;
use axum::response::IntoResponse;
use axum::{extract::multipart::Field, http::header};
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::ClientBuilder;
use azure_storage_blobs::prelude::ContainerClient;
use futures_util::StreamExt;
use jsonwebtoken::{Validation, Algorithm, TokenData, decode};

pub struct File<'a> {
    pub name: String,
    pub content_type: String,
    pub field: Field<'a>,
}

#[derive(Clone)]
pub struct DocDepotService {
    pub container_client: ContainerClient,
}

impl DocDepotService {
    pub fn new(container_name: String) -> Self {
        Self {
            container_client: Self::get_container_client(container_name),
        }
    }

    pub fn get_container_client(container_name: String) -> ContainerClient {
        let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
        let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");

        let storage_credentials = StorageCredentials::Key(account.clone(), access_key);

        ClientBuilder::new(account, storage_credentials).container_client(container_name)
    }

    // Constuct a custom url for api.greenie.one and dev-api.greenie.one to download files
    pub fn constuct_url(container_name: String, file_name: String) -> String {
        let env = std::env::var("APP_ENV").unwrap();
        let url = match env.as_str() {
            "dev" => format!(
                "https://dev-api.greenie.one/utils/doc_depot/{}/{}",
                container_name, file_name
            ),
            _ => format!(
                "https://api.greenie.one/utils/doc_depot/{}/{}",
                container_name, file_name
            ),
        };
        url
    }

    pub fn validate_token(token: String) -> APIResult<String> {
        let validation = Validation::new(Algorithm::RS256);
        let token_claims: TokenData<DownloadToken> = decode(token.as_ref(), &DECODE_KEY, &validation)?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if token_claims.claims.exp < now {
            return Err(APIError::TokenExpired);
        }

        Ok(token_claims.claims.url)
    }
}

impl DocDepotService {
    pub async fn upload_file<'a>(&mut self, file: File<'a>) -> APIResult<String> {
        let file_name = &file.name;
        let content_type = &file.content_type;

        let blob_client = self.container_client.blob_client(file_name);

        if blob_client.exists().await? {
            return Err(APIError::FileAlreadyExists);
        }

        blob_client
            .put_block_blob(file.field.bytes().await?)
            .content_type(content_type)
            .await?;

        let container_name = self.container_client.container_name();
        Ok(Self::constuct_url(
            container_name.to_string(),
            file_name.to_string(),
        ))
    }

    pub async fn download_file(&self, file_name: String) -> APIResult<impl IntoResponse> {
        let blob_client = self.container_client.blob_client(file_name);

        if blob_client.exists().await? == false {
            return Err(APIError::FileNotFound);
        }

        let mut data = blob_client.get().into_stream();
        let blob = data
            .next()
            .await
            .ok_or_else(|| APIError::InternalServerError("No data found".to_string()))??;

        let content_type = blob.blob.properties.content_type;
        let file_name = blob.blob.name;
        let stream_body = StreamBody::new(blob.data);
        let headers = [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", file_name),
            ),
        ];
        Ok((headers, stream_body))
    }
}
