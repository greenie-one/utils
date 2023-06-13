use crate::{errors::Result};
use axum::extract::multipart::Field;
use azure_core::Url;
use azure_storage_blobs::prelude::{BlobBlockType, BlockList, ContainerClient};

pub struct File<'a> {
    pub name: String,
    pub content_type: String,
    pub field: Field<'a>,
}

pub async fn upload_file_chunked<'a>(
    mut file: File<'a>,
    container_client: ContainerClient,
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

pub async fn delete_file(file_name: &str, container_client: ContainerClient) -> Result<()> {
    let blob_client = container_client.blob_client(file_name);
    blob_client.delete().await?;
    Ok(())
}
