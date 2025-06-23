use axum::Json;
use serde::Serialize;

use crate::models::{chapter::Chapter, contact::ContactInfo, event::Event, project::Project, resource::Resource};

#[derive(Serialize)]
pub struct HomePageData {
    pub about_community: String,
    pub socials: Vec<String>,
    pub contacts: ContactInfo,
    pub projects: Vec<Project>,
    pub events: Vec<Event>,
    pub chapters: Vec<Chapter>,
    pub resources: Vec<Resource>,
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
        },

        projects: vec![
            Project {
                uuid: "67e55044-10b1-426f-9247-bb680e5fe0c8".to_string(),
                name: "Rust Jobs Board".to_string(),
                description: "A curated job board for Rust developers in Kenya.".to_string(),
                repo_url: Some("https://github.com/RustaceansKenya/jobs-board".to_string()),
            },
            Project {
                uuid: "67e55044-10b1-426f-9247-bb680e5fe0c8".to_string(),
                name: "Beginner Rust Book Club".to_string(),
                description: "A collaborative reading group for beginners learning Rust.".to_string(),
                repo_url: None,
            },
        ],

        events: vec![
            Event {
                name: "The Borrow Checker".to_string(),
                description: "Understand the semantics and debug the borrow checker".to_string(),
                event_url: None,
            }
        ],

        chapters: vec![
            Chapter { region: "Nairobi".to_string() },
            Chapter { region: "Kisumu".to_string() },
            Chapter { region: "Kilifi".to_string() },
        ],

        resources: vec![
            Resource { name: "Get started with Rust".to_string(), hyperlink: "https://getstartedwith.rs".to_string()},
            Resource { name: "Get started with Rust".to_string(), hyperlink: "https://getstartedwith.rs".to_string()},
            Resource { name: "Get started with Rust".to_string(), hyperlink: "https://getstartedwith.rs".to_string()},
        ]
    };

    Json(data)
}