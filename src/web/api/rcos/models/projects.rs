//! Models relating to projects registered with Telescope.

use chrono::{DateTime, Utc};

/// An RCOS project as stored on the central database.
#[derive(Deserialize, Clone, Debug)]
pub struct Project {
    /// The project identifier.
    project_id: i64,
    /// The name of the project.
    title: String,
    /// The project description in markdown.
    description: String,
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
    /// The ID of an associated external organization if there is one.
    external_organization_id: Option<i64>,
}
