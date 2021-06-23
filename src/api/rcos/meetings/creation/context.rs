//! GraphQL query to get context for meeting creation.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

// /// ZST representing the GraphQL query to resolve meeting creation context.
// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "graphql/rcos/schema.json",
//     query_path = "graphql/rcos/meetings/creation/context.graphql",
//     response_derives = "Debug,Clone,Serialize"
// )]
// pub struct CreationContext;

// impl CreationContext {
//     /// Get the meeting creation context.
//     pub async fn get(host: Option<String>) -> Result<creation_context::ResponseData, TelescopeError> {
//
//         send_query::<Self>(creation_context::Variables {
//             now: chrono::Utc::today().naive_utc(),
//             host_username: host.map(||)
//         })
//         .await
//     }
// }
