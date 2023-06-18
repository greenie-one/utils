use axum::{extract::DefaultBodyLimit, Router};
use azure_storage_blobs::prelude::ContainerClient;
use mongodb::Client;

use crate::{
    handlers::profile_picture::{upload, delete},
    state::{
        blob_storage::{self, ContainerType},
        db::Database,
    },
};

const MAX_SIZE: usize = 4 * 1024 * 1024; // max payload size is 4MB

#[derive(Clone)]
pub struct AppState {
    pub db: Client,
    pub container_client: ContainerClient,
}

pub async fn routes() -> Router {
    let db = Database::get_client().await.unwrap();
    let blob_storage = blob_storage::get_container_client(ContainerType::Images);

    let state = AppState {
        db,
        container_client: blob_storage,
    };
    println!("Mapping profile picture routes");
    axum::Router::new()
        .route("/profile_pic", axum::routing::post(upload)).route("/profile_pic", axum::routing::delete(delete))
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_SIZE))
}
