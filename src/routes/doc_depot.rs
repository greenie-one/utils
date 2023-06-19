use axum::{extract::DefaultBodyLimit, Router};
use tracing::info;

use crate::{
    handlers::doc_depot::upload, state::app_state::FileHandlerState,
};

const MAX_SIZE: usize = 10 * 1024 * 1024; // max payload size is 4MB

pub async fn routes() -> Router {
    let redis_client = crate::state::redis::get_client();
    let state = FileHandlerState {
        redis_client,
    };
    info!("Mapping Doc Depot routes - POST /doc_depot");
    axum::Router::new()
        .route("/doc_depot", axum::routing::post(upload))
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_SIZE))
}
