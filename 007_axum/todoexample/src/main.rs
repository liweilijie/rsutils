use axum::body::HttpBody;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::middleware::AddExtension;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    Extension,
};
use axum::{Json, Router, Server, TypedHeader};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::RwLock;
use std::time::{Duration, SystemTime};

const SECRET_KEY: &[u8] = b"deadlock";
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub user_id: usize,
    pub title: String,
    pub completed: bool,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    name: String,
    exp: usize,
}

#[derive(Debug, Default, Clone)]
struct TodoStore {
    items: Arc<RwLock<Vec<Todo>>>,
}

#[tokio::main]
async fn main() {
    let store = TodoStore {
        items: Arc::new(RwLock::new(vec![
            Todo {
                id: 0,
                user_id: 0,
                title: "Learn Rust".to_string(),
                completed: false,
            },
            Todo {
                id: 100,
                user_id: 100,
                title: "Learn Axum".to_string(),
                completed: false,
            },
        ])),
    };

    let app = Router::new()
        .route("/", get(index_handler))
        .route(
            "/todos",
            get(todos_handler)
                .post(create_todo_handler)
                .layer(Extension(store)),
        )
        .route("/login", post(login_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("Hello, world!")
}

async fn todos_handler(
    claim: Claims,
    Extension(store): Extension<TodoStore>,
) -> Result<Json<Vec<Todo>>, HttpError> {
    let user_id = claim.id; // ???????????????id, ????????????????????????
    match store.items.read() {
        Ok(items) => Ok(Json(
            items
                // ??????????????????????????? ????????????????????????
                // .clone()
                // .into_iter()
                // .filter(|todo| todo.user_id == user_id)
                // .collect(),
                .iter()
                .filter(|todo| todo.user_id == user_id)
                .map(|todo| todo.clone()) // ??????filter+map ????????????????????????????????? map ???????????? clone??????
                .collect(),
        )),
        Err(_) => Err(HttpError::Internal),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodo {
    title: String,
}

// Claims need implementation FromRequest trait
async fn create_todo_handler(
    claim: Claims,
    Json(todo): Json<CreateTodo>,
    Extension(store): Extension<TodoStore>,
) -> Result<StatusCode, HttpError> {
    println!("claim: {claim:?}");
    println!("{todo:?}");

    match store.items.write() {
        Ok(mut guard) => {
            let todo = Todo {
                id: get_next_id(),
                user_id: claim.id,
                title: todo.title,
                completed: false,
            };

            guard.push(todo);

            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            println!("{}", e);
            Err(HttpError::Internal)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IkpvaG4gRG9lIiwiZXhwIjoxNjUxNjM0MDk4fQ.4QtlPan7O7Nbf101GNdeYV1FvOAefE3PoRetGILp-8g
async fn login_handler(Json(login): Json<LoginRequest>) -> Json<LoginResponse> {
    println!("{:?}", login);
    // TODO: validate email and password

    let claims = Claims {
        id: 1,
        name: "John Doe".to_string(),
        exp: get_epoch(),
    };

    // TODO: generate token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .unwrap();
    Json(LoginResponse { token })
}

fn get_epoch() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 14 * 86400
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send, // required by `async_trait`
{
    type Rejection = HttpError; // ???????????? IntoResponse

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| HttpError::Auth)?;

        // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
        // let token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
        let key = DecodingKey::from_secret(SECRET_KEY);
        let token =
            decode::<Claims>(bearer.token(), &key, &Validation::default()).map_err(|e| {
                println!("{:?}", e);
                HttpError::Auth
            })?;

        Ok(token.claims)
    }
}

#[derive(Debug)]
enum HttpError {
    Auth,
    Internal,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };

        // tuple????????????????????????????????? IntoResponse, ???????????? tuple ???????????? IntoResponse
        (code, msg).into_response()
    }
}

fn get_next_id() -> usize {
    // fetch_add??????????????????????????????????????????1??????
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
