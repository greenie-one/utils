use axum::{routing::get, Router};
use std::{net::SocketAddr, thread, time::Duration};
use tokio::time;
use tracing::{info, log::error, Level};
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

use crate::services::file_cleanup_job::cleanup;
pub(crate) mod database;
pub(crate) mod dtos;
pub(crate) mod env_config;
pub(crate) mod errors;
pub(crate) mod handlers;
pub(crate) mod routes;
pub(crate) mod services;
pub(crate) mod state;

const CRON_TIME_INTERVAL_IN_SEC: u64 = 60 * 60 * 24;

pub async fn build_run() {
    env_config::load_env();

    let tracing_layer = tracing_subscriber::fmt::layer();

    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let cron = tokio::task::spawn(async {
                    info!("Started cleanup job cron job");
                    let mut interval = time::interval(Duration::from_secs(CRON_TIME_INTERVAL_IN_SEC));
                    loop {
                        interval.tick().await;
                        let res = cleanup().await;
                        if let Err(e) = res {
                            error!("Cleanup job failed: {}", e.to_string());
                        }
                    }
                });
                let _ = cron.await;
            });
    });

    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    let pf_routes = routes::profile_picture::routes().await;
    let doc_depot_routes = routes::doc_depot::routes().await;
    let app = Router::new()
        .merge(pf_routes)
        .merge(doc_depot_routes)
        .route("/health-check", get(|| async { "All Ok!" }))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    info!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    print!("Server stopped");
}
