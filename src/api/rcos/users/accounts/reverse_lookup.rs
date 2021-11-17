//! RCOS API query to get the user ID (if available) of a user by platform and account id.

// Import and rename for GraphQL macro
use crate::api::rcos::prelude::*;
use crate::api::rcos::users::UserAccountType as user_account;

/// Type representing query for user ID given a platform and user id on that
/// platform.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/reverse_lookup.graphql"
)]
pub struct ReverseLookup;

use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use reverse_lookup::ResponseData;
use reverse_lookup::Variables;

impl ReverseLookup {
    /// Make the variables for a reverse account lookup.
    fn make_vars(platform: user_account, platform_id: String) -> Variables {
        Variables {
            platform,
            id: platform_id,
        }
    }

    /// Get the user ID associated with an ID on a different platform if available.
    pub async fn execute(
        platform: user_account,
        platform_id: String,
    ) -> Result<Option<uuid>, TelescopeError> {
        send_query::<Self>(Self::make_vars(platform, platform_id))
            .await
            .map(|response| response.user_id())
    }
}

impl ResponseData {
    /// Get the user ID of a user (if they exist) via their
    /// account id for a given platform.
    fn user_id(mut self) -> Option<uuid> {
        Some(self.user_accounts.pop()?.user_id)
    }
}
