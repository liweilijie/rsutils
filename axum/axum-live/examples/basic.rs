use std::net::SocketAddr;
use anyhow::Result;
use axum::{Json, Router, Server};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use tracing::info;
use serde::{Serialize, Deserialize};
use jsonwebtoken as jwt;

const SECRET: &[u8] = b"deadbeef";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/todos", get(todos_handler).post(create_todo_handler))
        .route("/login", post(login_handler)
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Listening on http://{}", addr);

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

async fn index_handler() -> Html<&'static str> {
    Html("Hello, world!")
}

async fn todos_handler() -> Json<Vec<Todo>> {
    Json(vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Learn Axum".to_string(),
            completed: false,
        },
    ])
}

async fn create_todo_handler(Json(todo): Json<CreateTodo>) -> StatusCode {
    info!("Created todo: {:?}", todo);
    StatusCode::CREATED
}

async fn login_handler(Json(_): Json<LoginRequest>) -> Json<LoginResponse> {
    let claims = Claims {
        id: 1,
        name: "liwei".to_string(),
    };

    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
   token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    name: String,
}

