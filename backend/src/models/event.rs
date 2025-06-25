use serde::Serialize;

#[derive(Serialize)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub event_url: Option<String>,
}