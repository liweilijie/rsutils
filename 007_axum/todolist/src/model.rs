use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}


#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub completed: Option<bool>,
}

