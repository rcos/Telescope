//! RCOS API mutation to create a discord channel for a given projct id.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;

use crate::error::TelescopeError;

/// Type representing GraphQL mutation to create channel for a project.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_associations/small_group/create_small_group_channel.graphql"
)]
pub struct CreateOneSmallGroupChannel;

impl CreateOneSmallGroupChannel {
    pub async fn execute(
        small_group_id: i64,
        channel_id: String,
        kind: channel_type,
    ) -> Result<Option<String>, TelescopeError> {
        send_query::<Self>(create_one_small_group_channel::Variables {
            small_group_id,
            channel_id,
            kind,
        })
        .await
        .map(|response| {
            response
                .insert_small_group_channels_one
                .map(|obj| obj.channel_id)
        })
    }
}
