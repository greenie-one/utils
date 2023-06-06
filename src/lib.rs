use axum::{routing::get, Router};
use dotenv::dotenv;
use std::net::SocketAddr;

mod errors;
pub use errors::{Error, Result};
pub mod db;
pub mod dtos;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

pub async fn build_run() {
    dotenv().ok();

    // let db_client = db::Database::get_client().await.unwrap();

    let app = Router::new()
        .merge(routes::mailer::routes())
        .route("/health-check", get(|| async { "All Ok!" }));

    // let app = app.with_state(db_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    print!("Server stopped");
}
