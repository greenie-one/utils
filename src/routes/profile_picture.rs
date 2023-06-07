use axum::Router;

use crate::{handlers::profile_picture::upload, state::db::Database};

pub async fn routes() -> Router {
    let state = Database::get_client().await.unwrap();
    axum::Router::new()
        .route("/profile/upload", axum::routing::post(upload)).with_state(state)
}