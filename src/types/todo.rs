use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ToDo {
    pub id: Option<String>,
    pub task: Option<String>,
    pub completed: Option<bool>
}

impl ToDo {
    pub fn init() -> ToDo {
        ToDo { id: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()), task: None, completed: Some(false) }
    }
}
