
use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::api::rcos::projects::projects_page::{AllProjects, CurrentProjects};
use crate::error::TelescopeError;

/// ZST representing the associated GraphQL query.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/discord_generate.graphql",
    response_derives = "Debug,Clone,Serialize"
)]

pub struct DiscordGenerate;

impl DiscordGenerate{
    pub async fn get() -> Result<ResponseData, TelescopeError>{
            let project_id = CurrentProjects::get(0).map(|project| project.project_id);
            let project_channel_id = send_query(variables: T::Variables)
    }
}
*