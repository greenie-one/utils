use axum::{extract::DefaultBodyLimit, Router};
use tracing::info;

use crate::{
    state::{
        blob_storage::{self, ContainerType},
        db::Database, app_state::AppState,
    }, handlers::doc_depot::upload,
};

const MAX_SIZE: usize = 10 * 1024 * 1024; // max payload size is 4MB

pub async fn routes() -> Router {
    let db = Database::get_client().await.unwrap();
    let blob_storage = blob_storage::get_container_client(ContainerType::Files);

    let state = AppState {
        db,
        container_client: blob_storage,
    };

    info!("Mapping Doc Depot routes");
    axum::Router::new()
        .route("/doc_depot", axum::routing::post(upload))
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_SIZE))
}
