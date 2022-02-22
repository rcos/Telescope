use crate::api::rcos::{prelude::*, search_strings::resolve_search_string, send_query};
use crate::error::TelescopeError;
use crate::api::rcos::discord_assoications::project_info::projects::Variables;
use crate::api::rcos::discord_assoications::project_info::projects::ResponseData;

/// Projects per page.
const PER_PAGE: u32 = 20;

/// GraphQL query to get projects with enrollments in an ongoing semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_assoications/projects.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Projects;
impl Projects{
    pub async fn get(page : u32, search: Option<String>) -> Result<ResponseData, TelescopeError>{
        send_query::<Self>(Variables {
            limit: PER_PAGE as i64,
            offset: (PER_PAGE * page) as i64,
            search: resolve_search_string(search),
        })
        .await
    }
}

