use axum::{extract::DefaultBodyLimit, Router};
use azure_storage_blobs::prelude::ContainerClient;
use tracing::info;

use crate::{
    handlers::profile_picture::{upload},
    state::{
        blob_storage::{self, ContainerType},
    },
};

const MAX_SIZE: usize = 4 * 1024 * 1024; // max payload size is 4MB

#[derive(Clone)]
pub struct AppState {
    pub container_client: ContainerClient,
}

pub async fn routes() -> Router {
    let blob_storage = blob_storage::get_container_client(ContainerType::Images);

    let state = AppState {
        container_client: blob_storage,
    };
    info!("Mapping profile picture routes - POST /profile_pic");
    axum::Router::new()
        .route("/profile_pic", axum::routing::post(upload))
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_SIZE))
}
