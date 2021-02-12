//! Models relating to projects registered with Telescope.

use chrono::{DateTime, Utc};

#[derive(Deserialize, Clone, Debug)]
pub struct Project {
    project_id: i32,
    title: String,
    description: String,
    languages: Vec<String>,
    stack: Vec<String>,
    cover_image_url: Option<String>,
    homepage_url: Option<String>,
    repository_urls: Vec<String>,
    created_at: DateTime<Utc>,
}
