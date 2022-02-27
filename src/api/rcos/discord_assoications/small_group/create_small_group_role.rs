//! RCOS API mutation to create a discord channel for a given projct id.

use crate::api::rcos::send_query;

use crate::error::TelescopeError;

/// Type representing GraphQL mutation to create channel for a project.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_assoications/small_group/create_small_group_role.graphql"
)]
pub struct CreateOneSmallGroupRole;

impl CreateOneSmallGroupRole {
    pub async fn execute(
        small_group_id: i64,
        role_id: String,
    ) -> Result<Option<String>, TelescopeError> {
        send_query::<Self>(create_one_small_group_role::Variables {
            small_group_id,
            role_id,
        })
        .await
        .map(|response| response.insert_small_group_roles_one.map(|obj| obj.role_id))
    }
}
