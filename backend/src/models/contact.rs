use serde::Serialize;

#[derive(Serialize)]
pub struct ContactInfo {
    pub email: String,
}