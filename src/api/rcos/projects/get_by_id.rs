//! GraphQL query to get a project by its ID.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing public RCOS projects.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/get_by_id.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Project;

use self::project::{ProjectProject, Variables};

impl Project {
    /// Get a Project by its ID.
    pub async fn get(project_id: i64) -> Result<Option<ProjectProject>, TelescopeError> {
        Ok(send_query::<Self>(Variables { id: project_id })
            // Wait for API response
            .await?
            // Extract the project object.
            .project)
    }
}

impl ProjectProject{
    /// Get the title of this project. This is the user-defined title if there is one, otherwise
    /// a title is constructed from the start date and project type.
    pub fn title(&self) -> String {
        self.title.clone()
    }
}
