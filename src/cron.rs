use std::collections::{HashMap, HashSet};

use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::BlobServiceClient;
use futures_util::StreamExt;
use mongodb::bson::Document;
use tracing::log::info;

use crate::errors::server_errors::ServerError;
use crate::{database::mongo::MongoDB, errors::server_errors::ServerResult};

use crate::utils::checks::is_object_id;

pub async fn fetch_container_names() -> ServerResult<Vec<String>> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");
    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);

    let mut list_conatiner_stream = blob_service.list_containers().into_stream();

    let mut container_names_all: Vec<String> = vec![];
    while let Some(containers_list) = list_conatiner_stream.next().await {
        let containers = containers_list?.containers;
        let container_names: Vec<String> = containers
            .iter()
            .filter(|c| is_object_id(&c.name))
            .map(|c| c.name.clone())
            .collect();
        container_names_all.extend(container_names);
    }
    return Ok(container_names_all);
}

pub async fn list_blob_names(
    container_names: Vec<String>,
) -> ServerResult<HashMap<String, HashSet<String>>> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");
    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);

    let mut files: HashMap<String, HashSet<String>> = HashMap::new();
    for container_name in container_names {
        let mut list_blob_stream = blob_service
            .container_client(&container_name)
            .list_blobs()
            .into_stream();

        while let Some(blobs_list) = list_blob_stream.next().await {
            let blobs_list = blobs_list?;
            let blobs = blobs_list.blobs.blobs();
            let blob_names: Vec<String> = blobs.into_iter().map(|b| b.name.clone()).collect();
            files
                .entry(container_name.clone())
                .or_insert(HashSet::new())
                .extend(blob_names);
        }
    }
    return Ok(files);
}

pub async fn files_from_private_urls() -> ServerResult<HashMap<String, HashSet<String>>> {
    let client = MongoDB::new().await;
    let documents = client.connection.collection::<Document>("documents");
    // fetch all documents
    let mut cursor = documents.find(None, None).await?;
    let mut files: HashMap<String, HashSet<String>> = HashMap::new();
    while let Some(result) = cursor.next().await {
        let document = result?;
        let url = document.get_str("privateUrl")?;
        let filename = url.split("/").last().unwrap();
        let container = document.get_str("user")?;
        files
            .entry(container.to_string())
            .or_insert(HashSet::new())
            .insert(filename.to_string());
    }
    return Ok(files);
}

pub async fn delete_containers(container_names: Vec<String>) {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");
    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);

    for container_name in container_names {
        info!("deleting container: {}", container_name);
        let _ = blob_service.container_client(container_name).delete().await;
    }
}

pub async fn delete_blobs(container_name: String, blob_names: Vec<String>) {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");
    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);

    for blob_name in blob_names {
        info!("deleting blob: {}", blob_name);
        let _ = blob_service
            .container_client(&container_name)
            .blob_client(&blob_name)
            .delete()
            .await;
    }
}

pub async fn cleanup() -> ServerResult<()> {
    let storage_container_names = fetch_container_names().await?;
    let db_files = files_from_private_urls().await?;
    let storage_container_names_set: HashSet<String> =
        storage_container_names.iter().map(|f| f.clone()).collect();

    let db_container_names_set: HashSet<String> = db_files.iter().map(|f| f.0.clone()).collect();
    let difference = storage_container_names_set.difference(&db_container_names_set);

    let to_delete_containers: Vec<String> = difference.map(|f| f.clone()).collect();
    let handle = tokio::spawn(delete_containers(to_delete_containers));

    let mut join_set = tokio::task::JoinSet::new();
    let stored_files = list_blob_names(db_container_names_set.into_iter().collect()).await?;
    for (container_name, stored_file_names) in stored_files {
        let db_file_names = db_files.get(&container_name).ok_or_else(|| {
            ServerError::AzureError(format!(
                "No user found for container: {}",
                container_name
            ))
        })?;

        let difference = stored_file_names.difference(&db_file_names);
        let to_delete_files: Vec<String> = difference.map(|f| f.clone()).collect();
        join_set.spawn(delete_blobs(container_name, to_delete_files));
    }

    handle.await.unwrap();
    while let Some(_) = join_set.join_next().await {}
    Ok(())
}
