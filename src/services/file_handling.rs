use crate::{
    dtos::{self, redis_values::FileStatus},
    errors::Result,
    state::redis::get_client,
};
use axum::extract::multipart::Field;
use azure_core::Url;

use azure_storage_blobs::prelude::{BlobBlockType, BlockList, ContainerClient};
use redis::Commands;

use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::ClientBuilder;
use tracing::log::{info, warn};

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

pub async fn upload_file_chunked<'a>(
    file: &mut File<'a>,
    container_client: &mut ContainerClient,
) -> Result<Url> {
    let file_name = &file.name;
    let content_type = &file.content_type;

    let blob_client = container_client.blob_client(file_name);
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

// pub async fn upload_file<'a>(file: File<'a>, container_client: ContainerClient) -> Result<Url> {
//     let file_name = file.name;
//     let content_type = file.content_type;

//     let blob_client = container_client.blob_client(file_name);
//     blob_client
//         .put_block_blob(file.field.bytes().await?)
//         .content_type(content_type)
//         .await?;
//     Ok(blob_client.url()?)
// }

pub async fn delete_file(file_name: &str, container_client: ContainerClient) -> Result<()> {
    let blob_client = container_client.blob_client(file_name);
    blob_client.delete().await?;
    Ok(())
}

pub fn monitor_file_commit(
    file_name: String,
    container_client: ContainerClient,
    mut redis_client: redis::Client,
    url: String,
    monitor_time_in_secs: u64,
) {
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(monitor_time_in_secs)).await;
        let value: Option<String> = redis_client.get(url.as_str()).unwrap();
        match value {
            Some(val) => {
                let obj: FileStatus = serde_json::from_str(val.as_str()).unwrap();
                if !obj.commited {
                    warn!(
                        "{}",
                        format!(
                            "File not commited, deleting file, key found, file: {}, url: {}",
                            file_name, url
                        )
                    );
                    delete_file(file_name.as_str(), container_client)
                        .await
                        .unwrap();
                }
            }
            None => {
                warn!(
                    "{}",
                    format!(
                        "File not commited, deleting file, key not found, file: {}, url: {}",
                        file_name, url
                    )
                );
                delete_file(file_name.as_str(), container_client)
                    .await
                    .unwrap();
            }
        }
        let _: i32 = redis_client.del(url.as_str()).unwrap();
    });
}

pub async fn delete_message_consumer() {
    let redis_client = get_client();
    let mut con = redis_client.get_connection().unwrap();
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("doc_delete").unwrap();
    loop {
        let msg = pubsub.get_message().unwrap();
        let channel = msg.get_channel_name();
        let payload: String = msg.get_payload().unwrap();
        info!("Delete Doc -> Channel: {}, Payload: {}", channel, payload);
        let delete_doc =
            serde_json::from_str::<dtos::redis_values::FileDeleteRequest>(&payload).unwrap();
        let container_client = get_container_client(delete_doc.container_name.clone());
        delete_file(delete_doc.file_name.as_str(), container_client)
            .await
            .unwrap();
    }
}
