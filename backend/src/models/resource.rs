use serde::Serialize;

#[derive(Serialize)]
pub struct Resource {
    pub name: String,
    pub hyperlink: String,
}