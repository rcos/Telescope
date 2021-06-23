//! GraphQL query to get host selection options ans suggestions.

use crate::api::rcos::prelude::*;
use crate::api::rcos::search_strings::resolve_search_string;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use chrono::Utc;

/// Type representing host selection GraphQL query.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/creation/host_selection.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct HostSelection;

impl HostSelection {
    /// Get the host selection data from the RCOS API.
    pub async fn get(
        search: Option<String>,
    ) -> Result<host_selection::ResponseData, TelescopeError> {
        send_query::<Self>(host_selection::Variables {
            search: resolve_search_string(search),
            now: Utc::today().naive_utc(),
        })
        .await
    }
}
