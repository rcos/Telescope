//! RCOS API mutation to create a discord channel for a given projct id.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;

use crate::error::TelescopeError;

/// Type representing GraphQL mutation to create channel for a project.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_assoications/create_project_category.graphql",
)]
pub struct CreateOneProjectCategory;


impl CreateOneProjectCategory{
    pub async fn execute(project_id : i64,  category_id: String) -> Result<Option<String>, TelescopeError>{
        send_query::<Self>(create_one_project_category::Variables{
            project_id,
            category_id,
        }).await
        .map(|response| response.insert_project_categories_one.map(|obj| obj.category_id))
        
    }
}