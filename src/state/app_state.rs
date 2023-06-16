use azure_storage_blobs::prelude::ContainerClient;
use mongodb::Client;

#[derive(Clone)]
pub struct AppState {
    pub db: Client,
    pub container_client: ContainerClient,
}