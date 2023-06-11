use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::{ClientBuilder, ContainerClient};

fn get_storage_account_key() -> StorageCredentials {
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");

    StorageCredentials::Key(account.clone(), access_key)
}

pub enum ContainerType {
    Images,
    Files,
}

pub fn get_container_client(container_name: ContainerType) -> ContainerClient {
    let value = match container_name {
        ContainerType::Images => "STORAGE_CONTAINER_IMAGES",
        ContainerType::Files => "STORAGE_CONTAINER_FILES",
    };
    let container = std::env::var(value).expect(format!("missing {}", value).as_str());

    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let storage_credentials = get_storage_account_key();

    ClientBuilder::new(account, storage_credentials).container_client(container)
}
