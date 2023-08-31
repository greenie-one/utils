use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::{ContainerClient, ClientBuilder};

use crate::env_config::{STORAGE_ACCOUNT, STORAGE_ACCESS_KEY};

pub fn get_container_client(container_name: &str) -> ContainerClient {
    let storage_credentials = StorageCredentials::Key(STORAGE_ACCOUNT.clone(), STORAGE_ACCESS_KEY.clone());
    ClientBuilder::new(STORAGE_ACCOUNT.clone(), storage_credentials).container_client(container_name)
}