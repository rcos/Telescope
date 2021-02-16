//! Models relating to projects registered with Telescope.

use chrono::{DateTime, Utc};

/// An RCOS project as stored on the central database.
#[derive(Deserialize, Clone, Debug)]
pub struct Project {
    /// The project identifier.
    project_id: i32,
    /// The name of the project.
    title: String,
    /// The project description in markdown.
    description: String,
    /// The programming languages used by the project.
    languages: Vec<String>,
    /// The technologies used by the project.
    stack: Vec<String>,
    /// The Url of the project's cover image or logo.
    cover_image_url: Option<String>,
    /// The Url of the project homepage.
    homepage_url: Option<String>,
    /// The urls of the projects repositories.
    repository_urls: Vec<String>,
    /// When the project was created.
    created_at: DateTime<Utc>,
}

/// Parameters for the API query to get projects.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ProjectQuery {
    
}
