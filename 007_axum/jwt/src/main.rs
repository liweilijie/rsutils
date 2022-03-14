extern crate core;
use anyhow::Result;

use std::net::SocketAddr;
use axum::{Router, Server};
use axum::routing::{get, post};

use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_jwt=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server at http://{}", addr);

    // build our application with a route
    let app = Router::new()
        .route("/protected", get(jwt::protected))
        .route("/authorize", post(jwt::authorize));

    debug!("Listening on http://{}", addr);

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
