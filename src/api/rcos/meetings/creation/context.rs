//! GraphQL query to get context for meeting creation.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use chrono::Utc;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/creation/context.graphql",
    response_derives = "Debug,Clone,Serialize",
    variables_derives = "Deserialize"
)]
pub struct CreationContext;

impl CreationContext {
    /// Get the meeting creation context.
    ///
    /// For meeting edits, semesters may be manually included by ID. otherwise, only ongoing and
    /// future semesters will be included.
    pub async fn execute(
        host: Option<uuid>,
        include_semesters: Vec<String>,
    ) -> Result<creation_context::ResponseData, TelescopeError> {
        send_query::<Self>(creation_context::Variables {
            host: host.map(|h| vec![h]).unwrap_or(vec![]),
            today: Utc::today().naive_utc(),
            include_semesters,
        })
        .await
    }
}
