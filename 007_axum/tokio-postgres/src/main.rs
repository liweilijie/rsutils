use anyhow::Result;
use axum::async_trait;
use axum::extract::{Extension, FromRequest, RequestParts};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Router, Server};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use std::net::SocketAddr;
use tokio_postgres::NoTls;
use tracing::{debug, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application.
struct DatabaseConnection(PooledConnection<'static, PostgresConnectionManager<NoTls>>);

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Staring tokio-postgres example");

    // setup connection pool
    let manager =
        PostgresConnectionManager::new_from_stringlike("host=localhost user=postgres", NoTls)
            .unwrap();

    let pool = Pool::builder().build(manager).await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route(
            "/",
            get(using_connection_pool_extractor).post(using_connection_extractor),
        )
        .layer(Extension(pool));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("Listening on http://{}", addr);

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

// we can exact the connection pool with `Extension`
async fn using_connection_pool_extractor(
    Extension(pool): Extension<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;

    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> std::result::Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<ConnectionPool>::from_request(req)
            .await
            .map_err(internal_error)?;

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;

    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}

/// Utility function for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))
}
