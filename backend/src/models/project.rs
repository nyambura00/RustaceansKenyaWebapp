use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub repo_url: Option<String>,
}