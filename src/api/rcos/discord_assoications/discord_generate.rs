/*
//! GraphQL query to get user info to populate the embed for the `/generate` command on the
//! RCOS Discord bot.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::api::rcos::projects::projects_page::{AllProjects, CurrentProjects};
use crate::error::TelescopeError;

/// ZST representing the associated GraphQL query.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_assoications/discord_generate.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct DiscordGenerate;

use discord_generate::ResponseData;
use discord_generate::Variables;

impl DiscordGenerate{
    pub async fn get() -> Result<ResponseData, TelescopeError>{
            let project_id = AllProjects::get(0, None).unwrap().map(|project| project.project_id);
            let project_info = send_query(variables: T::Variables{
                d
            })
    }
}
*/
