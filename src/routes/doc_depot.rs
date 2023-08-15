use axum::{extract::DefaultBodyLimit, Router};
use tracing::info;

use crate::{handlers::doc_depot::{upload, download}, state::app_state::UplaodState};

const MAX_SIZE: usize = 10 * 1024 * 1024; // max payload size is 4MB

pub async fn routes() -> Router {
    info!("Mapping Doc Depot routes - POST /doc_depot");
    let state = UplaodState::new().await;
    axum::Router::new()
        .route("/doc_depot", axum::routing::post(upload).with_state(state))
        .route("/doc_depot/:container_name/:file_name", axum::routing::get(download))
        .layer(DefaultBodyLimit::max(MAX_SIZE))
}
