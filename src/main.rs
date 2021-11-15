mod handlers;
mod utils;
use handlers::*;
use tracing::Level;
use utils::*;

use axum::{
    body::Body,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // .env var checks
    let port_env = env::var("PORT").expect("PORT must be set");
    let port = port_env.parse::<u16>().unwrap();

    // Declare API router and routes
    let app: Router<Body> = Router::new()
        .route("/", get(root))
        .route("/check", get(check_something))
        .route("/getBlockheight", get(fetch_blockheight_handler));

    // Bind server to PORT and serve the router
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::event!(Level::INFO, "Warlock started on port {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn check_something(Json(payload): Json<NetworkConfig>) -> impl IntoResponse {
    let x: NetworkConfig = payload.into();

    (StatusCode::OK, Json(x))
}
