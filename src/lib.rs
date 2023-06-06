use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

mod errors;
pub use errors::{Error, Result};
pub mod routes;
pub mod handlers;
pub mod dtos;
pub mod services;
pub mod models;
pub mod state;
pub mod env_config;

pub async fn build_run() {
    env_config::load_env();

    // let db_client = db::Database::get_client().await.unwrap();

    let app= Router::new().merge(routes::mailer::routes())
        .route("/health-check", get(|| async { "All Ok!" }));

    // let app = app.with_state(db_client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    print!("Server stopped");
}
