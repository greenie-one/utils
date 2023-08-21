use axum::Router;
use tracing::info;

use crate::{handlers::admin::create_account, state::app_state::AdminState};


pub async fn routes() -> Router {
    info!("Mapping Admin routes - POST /admin");
    let state = AdminState::new().await;
    axum::Router::new()
        .route("/admin/create_account", axum::routing::post(create_account)).with_state(state)
}
