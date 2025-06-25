use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Partnership {
    pub name: String,
    pub email: String,
    pub organization: String,
    pub message: Option<String>,
}