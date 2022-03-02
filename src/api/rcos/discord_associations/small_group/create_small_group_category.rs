//! RCOS API mutation to create a discord channel for a given projct id.

use crate::api::rcos::send_query;

use crate::error::TelescopeError;

/// Type representing GraphQL mutation to create channel for a project.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_associations/small_group/create_small_group_category.graphql"
)]
pub struct CreateOneSmallGroupCategory;

impl CreateOneSmallGroupCategory {
    pub async fn execute(
        small_group_id: i64,
        category_id: String,
    ) -> Result<Option<String>, TelescopeError> {
        send_query::<Self>(create_one_small_group_category::Variables {
            small_group_id,
            category_id,
        })
        .await
        .map(|response| {
            response
                .insert_small_group_categories_one
                .map(|obj| obj.category_id)
        })
    }
}
