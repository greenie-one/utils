use crate::{errors::Result, dtos::redis_values::FileStatus};
use axum::extract::multipart::Field;
use azure_core::Url;
use azure_storage::{
    prelude::BlobSasPermissions, shared_access_signature::service_sas::BlobSharedAccessSignature,
};
use azure_storage_blobs::prelude::{BlobBlockType, BlockList, ContainerClient};
use redis::Commands;
use time::OffsetDateTime;

use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::ClientBuilder;
use tracing::log::warn;

pub struct File<'a> {
    pub name: String,
    pub content_type: String,
    pub field: Field<'a>,
}

fn get_storage_account_key() -> StorageCredentials {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");

    StorageCredentials::Key(account.clone(), access_key)
}

pub fn get_container_client(container_name: String) -> ContainerClient {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let storage_credentials = get_storage_account_key();

    ClientBuilder::new(account, storage_credentials).container_client(container_name)
}

pub fn get_container_client_from_sas(container_name: String, sas_token: String) -> ContainerClient {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let storage_credentials = StorageCredentials::sas_token(sas_token).unwrap();

    ClientBuilder::new(account, storage_credentials).container_client(container_name)
}

pub async fn upload_file_chunked<'a>(
    mut file: File<'a>,
    container_client: &mut ContainerClient,
) -> Result<Url> {
    let file_name = file.name;
    let content_type = file.content_type;

    let blob_client = container_client.blob_client(file_name.as_str());
    let mut blocks = BlockList { blocks: Vec::new() };

    let mut chunk_id = 0;
    while let Some(chunk) = file.field.chunk().await? {
        let block_id = format!("{}{:03}", file_name, chunk_id);
        blob_client.put_block(block_id.clone(), chunk).await?;
        blocks.blocks.push(BlobBlockType::new_latest(block_id));
        chunk_id += 1;
    }

    blob_client
        .put_block_list(blocks)
        .content_type(content_type)
        .await?;

    Ok(blob_client.url()?)
}

pub async fn upload_file<'a>(file: File<'a>, container_client: ContainerClient) -> Result<Url> {
    let file_name = file.name;
    let content_type = file.content_type;

    let blob_client = container_client.blob_client(file_name);
    blob_client
        .put_block_blob(file.field.bytes().await?)
        .content_type(content_type)
        .await?;
    Ok(blob_client.url()?)
}

pub fn get_blob_sas(
    container_client: &mut ContainerClient,
    file_name: &str,
    expiry_in_mins: i64,
) -> Result<BlobSharedAccessSignature> {
    let blob_client = container_client.blob_client(file_name);
    let expiry = OffsetDateTime::now_utc() + time::Duration::minutes(expiry_in_mins);
    let sas = blob_client.shared_access_signature(
        BlobSasPermissions {
            read: true,
            ..BlobSasPermissions::default()
        },
        expiry,
    )?;

    Ok(sas)
}

pub fn get_container_sas(
    container_client: &mut ContainerClient,
    expiry_in_mins: i64,
) -> Result<BlobSharedAccessSignature> {
    let expiry = OffsetDateTime::now_utc() + time::Duration::minutes(expiry_in_mins);
    let sas = container_client.shared_access_signature(
        BlobSasPermissions {
            read: true,
            ..BlobSasPermissions::default()
        },
        expiry,
    )?;

    Ok(sas)
}

pub async fn delete_file(file_name: &str, container_client: ContainerClient) -> Result<()> {
    let blob_client = container_client.blob_client(file_name);
    blob_client.delete().await?;
    Ok(())
}

pub fn monitor_file_commit(
    file_name: String,
    container_client: ContainerClient,
    redis_client: redis::Client,
    url: String,
    monitor_time_in_secs: u64,
) {
    tokio::spawn(async move {
        let mut redis_client = redis_client.clone();
        tokio::time::sleep(tokio::time::Duration::from_secs(monitor_time_in_secs)).await;
        let value: Option<String> = redis_client.get(url.as_str()).unwrap();
        match value {
            Some(val) => {
                let obj: FileStatus = serde_json::from_str(val.as_str()).unwrap();
                if !obj.commited {
                    warn!("File not commited, deleting file");
                    delete_file(file_name.as_str(), container_client)
                        .await
                        .unwrap();
                }
            }
            None => {
                warn!("File not commited, deleting file, key not found");
                delete_file(file_name.as_str(), container_client)
                    .await
                    .unwrap();
            }
        }
        let _: i32 = redis_client.del(url.as_str()).unwrap();
    });
}
