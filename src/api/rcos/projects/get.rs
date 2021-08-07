//! List projects query.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use chrono::{DateTime, Utc};

/// Type representing public RCOS projects.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Projects;

use self::projects::{ProjectsProjects, Variables};

impl Projects {
    /// Get the projects between two times, optionally filter to finalized projects only.
    pub async fn get(
        limit: u32,
        offset: u32,
        search:&str,
    ) -> Result<Vec<ProjectsProjects>, TelescopeError> {
        Ok(send_query::<Self>(Variables {
            limit,
            offset,
            search,
        })
        .await?
        .AllProjects)
    }
}
