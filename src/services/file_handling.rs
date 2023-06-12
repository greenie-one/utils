use crate::{errors::Result, Error};
use axum::extract::multipart::Field;
use azure_storage_blobs::prelude::{BlobBlockType, BlockList, ContainerClient};
use reqwest::Url;

pub async fn upload_file_chunked<'a>(
    mut file: Field<'a>,
    container_client: ContainerClient,
) -> Result<Url> {
    let file_name = file
        .file_name()
        .ok_or_else(|| Error::InvalidFileName)?
        .to_string();
    let content_type = file
        .content_type()
        .ok_or_else(|| Error::InvalidContentType)?
        .to_string();
    println!("Uploading {} {}", file_name, content_type);

    let blob_client = container_client.blob_client(file_name.as_str());
    let mut blocks = BlockList { blocks: Vec::new() };

    let mut chunk_id = 0;
    while let Some(chunk) = file.chunk().await? {
        println!("Uploading chunk {}", chunk.len());
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

pub async fn validate_image_field(field: Field<'_>, field_name: String) -> Result<Field> {
    let file_name = field.file_name().ok_or_else(|| Error::InvalidFileName)?;
    let content_type = field
        .content_type()
        .ok_or_else(|| Error::InvalidContentType)?;
    let extracted_field_name = field.name().unwrap().to_string();

    println!(
        "Validating {} {} {}",
        file_name, content_type, extracted_field_name
    );

    if extracted_field_name != field_name {
        return Err(Error::InvalidFileName);
    }

    if !content_type.starts_with("image/") {
        return Err(Error::InvalidContentType);
    }

    if !file_name.ends_with(".jpg") && !file_name.ends_with(".jpeg") && !file_name.ends_with(".png")
    {
        return Err(Error::InvalidFileName);
    }

    Ok(field)
}
