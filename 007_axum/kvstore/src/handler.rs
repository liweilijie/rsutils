use crate::model::SharedState;
use axum::body::Bytes;
use axum::extract::{ContentLengthLimit, Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::delete;
use axum::{BoxError, Router};
use std::borrow::Cow;
use tower_http::auth::RequireAuthorizationLayer;

pub async fn kv_get(
    Path(key): Path<String>,
    Extension(state): Extension<SharedState>,
) -> Result<Bytes, StatusCode> {
    let db = &state.read().unwrap().db;

    if let Some(value) = db.get(&key) {
        Ok(value.clone())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn kv_set(
    Path(key): Path<String>,
    ContentLengthLimit(bytes): ContentLengthLimit<Bytes, { 1024 * 5_000 }>, // ~5mb
    Extension(state): Extension<SharedState>,
) {
    state.write().unwrap().db.insert(key, bytes);
}

pub async fn list_keys(Extension(state): Extension<SharedState>) -> String {
    let db = &state.read().unwrap().db;

    db.keys()
        .map(|key| key.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn admin_routes() -> Router {
    pub async fn delete_all_keys(Extension(state): Extension<SharedState>) {
        state.write().unwrap().db.clear();
    }

    pub async fn remove_key(Path(key): Path<String>, Extension(state): Extension<SharedState>) {
        state.write().unwrap().db.remove(&key);
    }

    Router::new()
        .route("/keys", delete(delete_all_keys))
        .route("/key/:key", delete(remove_key))
        // Require bearer auth for all admin routes
        .layer(RequireAuthorizationLayer::bearer("secret-token"))
}

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("server is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}
