use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HomePageData {
    pub about_community: String,
    pub socials: Vec<String>,
    pub contacts: ContactInfo,
    pub projects: Vec<Project>,
    pub events: Vec<Event>,
    pub chapters: Vec<String>,
    pub resources: Vec<String>,
}

#[derive(Serialize)]
pub struct ContactInfo {
    pub email: String,
    pub phone: String,
    pub location: String,
}

#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub repo_url: Option<String>,
}

#[derive(Serialize)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub event_url: Option<String>,
}

pub async fn get_homepage_data() -> Json<HomePageData> {
    let data = HomePageData {
        about_community: "Rustaceans Kenya is a welcoming space for Rust enthusiasts in Kenya to connect, learn, and grow together.".to_string(),

        socials: vec![
            "https://x.com/RustaceansKenya".to_string(),
            "https://github.com/RustaceansKenya".to_string(),
            "https://t.co/KNsybAQBKd".to_string(),
        ],

        contacts: ContactInfo {
            email: "hello@rustaceans.ke".to_string(),
            phone: "+254712345678".to_string(),
            location: "Nairobi, Kenya".to_string(),
        },

        projects: vec![
            Project {
                name: "Rust Jobs Board".to_string(),
                description: "A curated job board for Rust developers in Kenya.".to_string(),
                repo_url: Some("https://github.com/RustaceansKenya/jobs-board".to_string()),
            },
            Project {
                name: "Beginner Rust Book Club".to_string(),
                description: "A collaborative reading group for beginners learning Rust.".to_string(),
                repo_url: None,
            },
        ],

        events: vec![
            Event {
                name: "The Borrow Checker".to_string(),
                description: "Understand the semantics and debugging the borrow checker".to_string(),
                event_url: None,
            }
        ],

        chapters: vec![
            "Nairobi".to_string(),
            "Kisumu".to_string(),
            "Kilifi".to_string(),
        ],

        resources: vec![
            "Get started with Rust".to_string(),
            "Embedded development in Rust".to_string(),
            "Rustlang in Bitcoin".to_string(),
        ]
    };

    Json(data)
}