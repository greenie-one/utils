use axum::{extract::DefaultBodyLimit, Router};
use tracing::info;

use crate::{
    handlers::leads::upload,
    state::app_state::LeadState,
};

const MAX_SIZE: usize = 10 * 1024 * 1024; // max payload size is 10MB

pub async fn routes() -> Router {
    let state = LeadState::new();
    info!("Mapping profile picture routes - POST /leads");
    axum::Router::new()
        .route("/leads", axum::routing::post(upload))
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_SIZE))
}
