use std::{time::{SystemTime, UNIX_EPOCH}, any::Any, error::Error};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct ToDo {
    pub id: Option<String>,
    pub task: Option<String>,
    pub completed: Option<bool>,
    pub id_user: Option<String>,
}

impl ToDo {
    pub fn init() -> ToDo {
        ToDo { id: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()), task: None, completed: Some(false), id_user: None }
    }

    pub fn get_keys(&self) -> Vec<&str> {
        vec!["id", "task", "completed", "id_user"]
    }
}
