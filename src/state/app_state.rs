use azure_storage_blobs::prelude::ContainerClient;

#[derive(Clone)]
pub struct AppState {
    pub container_client: ContainerClient,
}

#[derive(Clone)]
pub struct FileHandlerState {
    pub redis_client: redis::Client,
}