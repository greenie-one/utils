use axum::Router;
use tracing::info;

use crate::{handlers::admin::create_hr_profile, state::app_state::AdminState};


pub async fn routes() -> Router {
    info!("Mapping Admin routes - POST /admin");
    let state = AdminState::new().await;
    axum::Router::new()
        .route("/admin/create_hr", axum::routing::post(create_hr_profile)).with_state(state)
}
