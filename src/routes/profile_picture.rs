use axum::{extract::DefaultBodyLimit, Router};
use tracing::info;

use crate::{
    handlers::profile_picture::{upload},
    state::{
        app_state::AppState,
    }, services::file_handling::get_container_client,
};

const MAX_SIZE: usize = 4 * 1024 * 1024; // max payload size is 4MB

pub async fn routes() -> Router {
    let blob_storage = get_container_client("images".into());

    let state = AppState {
        container_client: blob_storage,
    };
    info!("Mapping profile picture routes - POST /profile_pic");
    axum::Router::new()
        .route("/profile_pic", axum::routing::post(upload))
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_SIZE))
}
