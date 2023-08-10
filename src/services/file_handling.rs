use crate::errors::api_errors::APIResult;
use axum::{extract::multipart::Field, body::Bytes};
use azure_core::Url;

use azure_storage_blobs::prelude::ContainerClient;

use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::ClientBuilder;

pub struct File<'a> {
    pub name: String,
    pub content_type: String,
    pub field: Field<'a>,
}

pub fn get_container_client(container_name: String) -> ContainerClient {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");

    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);

    ClientBuilder::new(account, storage_credentials).container_client(container_name)
}

pub async fn upload_file<'a>(file: File<'a>, container_client: &mut ContainerClient) -> APIResult<Url> {
    let file_name = &file.name;
    let content_type = &file.content_type;

    let blob_client = container_client.blob_client(file_name);
    blob_client
        .put_block_blob(file.field.bytes().await?)
        .content_type(content_type)
        .await?;
    Ok(blob_client.url()?)
}