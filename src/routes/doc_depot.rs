use axum::{extract::DefaultBodyLimit, Router};
use tracing::info;

use crate::{
    handlers::doc_depot::upload,
};

const MAX_SIZE: usize = 10 * 1024 * 1024; // max payload size is 4MB

pub async fn routes() -> Router {

    info!("Mapping Doc Depot routes");
    axum::Router::new()
        .route("/doc_depot/:type", axum::routing::get(upload))
        .layer(DefaultBodyLimit::max(MAX_SIZE))
}
