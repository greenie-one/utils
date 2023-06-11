use crate::{errors::Result, Error};
use axum::extract::multipart::Field;
use azure_storage_blobs::prelude::{BlobBlockType, BlockList, ContainerClient};
use reqwest::Url;

pub async fn upload_file_chunked<'a>(
    mut file: Field<'a>,
    container_client: ContainerClient,
) -> Result<Url> {
    let file_name = file.file_name().ok_or_else(|| Error::InvalidFileName)?.to_string();
    let content_type = file.content_type().ok_or_else(|| Error::InvalidContentType)?.to_string();
    println!("Uploading {} {}", file_name, content_type);

    let blob_client = container_client.blob_client(file_name.as_str());
    let mut blocks = BlockList { blocks: Vec::new() };

    let mut chunk_id = 0;
    while let Some(chunk) = file.chunk().await? {
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

pub async fn upload_file<'a>(file: Field<'a>, container_client: ContainerClient) -> Result<Url> {
    let file_name = file.name().unwrap().to_string();
    let content_type = file.content_type().unwrap().to_string();
    println!("Uploading {} {}", file_name, content_type);

    let blob_client = container_client.blob_client(file_name);
    blob_client
        .put_block_blob(file.bytes().await?)
        .content_type(content_type)
        .await?;
    Ok(blob_client.url()?)
}
