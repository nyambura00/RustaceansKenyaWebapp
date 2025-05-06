mod routes;

use axum::{
    routing::get,
    Router,
};
use routes::{
    homepage::homepage, 
    workshop::book_workshop, 
    community::{projects::community_projects, chapters::community_chapters, events::community_events},
    learn_rust::learn_rust,
    contact_us::contact_us
};

#[tokio::main]
async fn main() {
    let community_routes = Router::new()
        .route("/projects", get(community_projects))
        .route("/chapters", get(community_chapters))
        .route("/events", get(community_events));

    let app = Router::new()
        .route("/", get(homepage))
        .route("/bookworkshop", get(book_workshop))
        .nest("/community", community_routes)
        .route("/learnrust", get(learn_rust))
        .route("/contactus", get(contact_us));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    axum::serve(listener, app).await.unwrap();
}
