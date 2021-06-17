//! GraphQL query to get context for meeting creation.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// ZST representing the GraphQL query to resolve meeting creation context.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/creation/context.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CreationContext;

use creation_context::String_comparison_exp as Filter;

impl Default for Filter {
    fn default() -> Self {
        Self {
            eq: None,
            gt: None,
            gte: None,
            ilike: None,
            in_: None,
            is_null: None,
            like: None,
            lt: None,
            lte: None,
            neq: None,
            nilike: None,
            nin: None,
            nlike: None,
            nsimilar: None,
            similar: None,
        }
    }
}

impl CreationContext {
    /// Get the meeting creation context.
    pub async fn get(host: Option<String>) -> Result<creation_context::ResponseData, TelescopeError> {
        // Convert host into filter to send in query.
        let filter = host
            .clone()
            .map(|username| Filter { eq: Some(username), ..Default::default() })
            .unwrap_or(Filter { is_null: Some(true), ..Default::default() });

        send_query::<Self>(creation_context::Variables {
            now: chrono::Utc::today().naive_utc(),
            host_filter: filter,
            host_username: host
        })
        .await
    }
}
