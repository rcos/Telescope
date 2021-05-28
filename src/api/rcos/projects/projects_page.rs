//! GraphQL query to get a paginated list of RCOS projects.

use crate::api::rcos::{prelude::*, search_strings::resolve_search_string, send_query};
use crate::error::TelescopeError;
use chrono::Utc;

/// Projects per page.
const PER_PAGE: u32 = 20;

/// GraphQL query to get projects with enrollments in an ongoing semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CurrentProjects;

/// GraphQL query to get all projects.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct AllProjects;

impl CurrentProjects {
    /// Get projects for a given page number (zero indexed).
    pub async fn get(page: u32, search: Option<String>) -> Result<current_projects::ResponseData, TelescopeError> {
        send_query::<Self>(current_projects::Variables {
            limit: PER_PAGE as i64,
            offset: (PER_PAGE*page) as i64,
            search: resolve_search_string(search),
            now: Utc::today().naive_utc(),
        }).await
    }
}

impl AllProjects {
    /// Get projects for a given page number (zero indexed).
    pub async fn get(page: u32, search: Option<String>) -> Result<all_projects::ResponseData, TelescopeError> {
        send_query::<Self>(all_projects::Variables {
            limit: PER_PAGE as i64,
            offset: (PER_PAGE*page) as i64,
            search: resolve_search_string(search)
        }).await
    }
}
