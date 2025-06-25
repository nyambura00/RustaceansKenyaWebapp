use axum::{Json, response::IntoResponse};
use http::StatusCode;
use super::model::Partnership;

pub async fn submit_partnership(Json(payload): Json<Partnership>) -> impl IntoResponse {
    println!("Received partnership: {:?}", payload);

    // TODO: Store it in a database or send an email notification

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": "Thank you for your interest in partnering with us!"
        }))
    )
}

pub async fn get_partnerships() -> impl IntoResponse {
    let mock_data = vec![
        Partnership {
            name: "Jane Doe".into(),
            email: "jane@example.com".into(),
            organization: "Rust Women KE".into(),
            message: Some("Looking to collaborate.".into()),
        }
    ];

    Json(mock_data)
}