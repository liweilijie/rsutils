use std::net::SocketAddr;
use std::time::Duration;
use anyhow::Result;
use axum::error_handling::HandleErrorLayer;
use axum::extract::Extension;
use axum::{Router, Server};
use axum::routing::{get, patch};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use todolist::{Db, handle_error, todos_create, todos_delete, todos_index, todos_update};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting example_todos");

    let db = Db::default();

    // Compose the routes
    let app = Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
    // Add middleware to all routes
        .layer(
         ServiceBuilder::new()
             .layer(HandleErrorLayer::new(handle_error))
             .timeout(Duration::from_secs(10))
             .layer(TraceLayer::new_for_http())
             .layer(Extension(db))
             .into_inner(),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("Listening on http://{}", addr);

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}