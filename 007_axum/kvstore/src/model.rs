use axum::body::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct State {
    pub db: HashMap<String, Bytes>,
}

pub type SharedState = Arc<RwLock<State>>;
