use crate::api::rcos::{prelude::*, search_strings::resolve_search_string, send_query};
use crate::error::TelescopeError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/projects/project_channels.graphql",
    response_derives = "Debug,Clone,Serialize"
)]

pub struct ProjectChannels;

impl ProjectChannels{
    // Get project channels for a given page nunmber (zero indexed).
    pub async fn get(
        page: u32, 
        search: Option<String>,
    ) -> Result<project_channels::ResponseData, TelescopeError>{
        send_query::<Self>(project:channels::Variables {
            pr
        })
    }
}