use std::net::SocketAddr;
use std::sync::Arc;
use arc_swap::ArcSwap;
use axum::response::IntoResponse;
use axum::{Extension, Router};
use axum::routing::{get, post};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use arcswap::config::{ParamsConfig, ServerConfig};

// ArcSwap 里面包含的是一个 Arc<T>
// 因为 Extension里面的数据需要实现 Clone 方法，所以这里必须要用 Arc 将 ArcSwap再包一层才有 Clone
type ArcParams = Arc<ArcSwap<ParamsConfig>>;

#[tokio::main]
async fn main() {
    // init log
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "arcswap=debug".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = ServerConfig::load().await;

    // ArcSwap定义里面需要一个 Arc<T>的参数
    let params: ArcParams = Arc::new(arc_swap::ArcSwap::new(Arc::new(config.params)));

    // 让 network 实现了一个 `impl From<NetworkConfig> SocketAddr`
    let addr: SocketAddr = config.network.into();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/reload", post(reload_handler))
        .layer(Extension(params));

     info!("Listening on http://{addr:?}");

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

// 每次从 ArcSwap 中获取数据
async fn index_handler(Extension(params): Extension<ArcParams>) -> impl IntoResponse {
    let p = params.load();
    format!("params: {p:?}")
}

// 当发送 post 的时候，再次读取配置文件，并且将读取的内容 store 到 ArcSwap 中
async fn reload_handler(Extension(params): Extension<ArcParams>) -> impl IntoResponse {
    // reload config
    let new_config = ServerConfig::load().await;
    params.store(Arc::new(new_config.params));
    format!("reload config successfully")
}