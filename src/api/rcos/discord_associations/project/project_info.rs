use crate::api::rcos::{prelude::*, search_strings::resolve_search_string, send_query};
use crate::error::TelescopeError;
use chrono::Utc;

/// Projects per page.
const PER_PAGE: u32 = 20;
/// GraphQL query to get projects with enrollments in an ongoing semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_associations/project/projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CurrProjects;

impl CurrProjects {
    pub async fn get(
        page: u32,
        search: Option<String>,
    ) -> Result<curr_projects::ResponseData, TelescopeError> {
        send_query::<Self>(curr_projects::Variables {
            offset: (PER_PAGE * page) as i64,
            search: resolve_search_string(search),
            now: Utc::today().naive_utc(),
        })
        .await
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_associations/project/find_project.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct FindProject;

impl FindProject {
    pub async fn get_by_id(id: i64) -> Result<find_project::ResponseData, TelescopeError> {
        send_query::<Self>(find_project::Variables {
            id: id,
        })
        .await
    }
}
