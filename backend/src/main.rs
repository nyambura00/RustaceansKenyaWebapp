mod routes;
mod models;
mod db;

use db::init::setup_db;
use axum::{
    routing::{get, get_service, post},
    Router,
    Extension,
};
use routes::{
    homepage::get_homepage_data,
    workshops::book_workshop,
    partnerships::{get_partnerships, submit_partnership},
    contact_us::contact_us
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frontend_dir = get_service(ServeDir::new("frontend/dist")).handle_error(|err| async move {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    });

    let db_state = setup_db().await?;

    let app = Router::new()
        .fallback_service(frontend_dir) // serve frontend build directory

        .layer(Extension(db_state))

        .route("/home", get(get_homepage_data))

        .route("/workshop", post(book_workshop))

        .route("/partnerships", get(get_partnerships).post(submit_partnership))

        .route("/contactus", post(contact_us))
        .nest_service("/contactus/assets", ServeDir::new("frontend/assets/contactus")); // TODO: set up spam checker

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
