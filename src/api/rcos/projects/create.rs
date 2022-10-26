//! GraphQL mutation to create a project.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use chrono::{DateTime, Utc};
use url::Url;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/create.graphql",
    response_derives = "Debug,Copy,Clone,Serialize"
)]
pub struct CreateProject;

/// Trim the whitespace off a string. If the trimmed string is empty default to None.
pub fn normalize_url(url: Option<String>) -> Option<String> {
    url.and_then(|string| (!string.trim().is_empty()).then(|| string))
}
impl CreateProject {
    /// Execute a Project creation mutation. Return the created Project's ID.
    pub async fn execute(
        title: Option<String>,
        stack: Option<Vec<String>>,
        repository_urls: Option<String>,
        homepage_url: Option<String>,
        description: Option<String>,
        cover_image_url: Option<String>,

    ) -> Result<Option<i64>, TelescopeError> {
        send_query::<Self>(create_project::Variables {
            title,
            stack,
            repository_urls,
            homepage_url,
            description,
            cover_image_url,
        })
        .await
        .map(|response| response.insert_projects_one.map(|obj| obj.project_id))
    }
}
