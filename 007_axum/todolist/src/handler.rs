use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tower::BoxError;
use uuid::Uuid;
use crate::{CreateTodo, Db, Pagination, Todo, UpdateTodo};

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        Ok(StatusCode::REQUEST_TIMEOUT)
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled interval error: {:?}", error),
        ))
    }
}

pub async fn todos_index(
    pagination: Option<Query<Pagination>>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {

    let todos = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}

pub async fn todos_create(
    Json(input): Json<CreateTodo>,
    Extension(db): Extension<Db>
) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}

pub async fn todos_update(
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
    Extension(db): Extension<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        todo.completed = completed;
    }

    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}

pub async fn todos_delete(
    Path(id): Path<Uuid>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {
    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
