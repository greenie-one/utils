use axum::{routing::get, Router};
use std::net::SocketAddr;

pub mod dtos;
pub mod env_config;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod services;
pub mod state;

use middleware as mw;

pub async fn build_run() {
    env_config::load_env();

    // let db_client = db::Database::get_client().await.unwrap();
    let pf_routes = routes::profile_picture::routes().await;
    let app = Router::new()
        .merge(pf_routes)
        .route("/health-check", get(|| async { "All Ok!" }))
        .layer(axum::middleware::from_fn(mw::logger::request_logger));

    // let app = app.with_state(db_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    print!("Server stopped");
}
