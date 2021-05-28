//! RCOS API query to get list of developers to display on the developers page.

use crate::api::rcos::{
    prelude::*,
    search_strings::resolve_search_string
};
use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use chrono::Utc;
use graphql_client::GraphQLQuery;

/// The query returns 20 developers per page.
pub const PER_PAGE: u32 = 20;

/// Type representing GraphQL query to get a list of all users and their
/// account associations for the developers page.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/developers.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct AllDevelopers;

/// Type representing the GraphQL query to get a list of currently enrolled
/// users and their account associations for the developers page.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/developers.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CurrentDevelopers;

impl AllDevelopers {
    /// Send the query to get all the developers (including old ones) and wait for a response.
    pub async fn get(
        page_num: u32,
        search: Option<String>,
    ) -> Result<<Self as GraphQLQuery>::ResponseData, TelescopeError> {
        send_query::<Self>(all_developers::Variables {
            limit: PER_PAGE as i64,
            offset: (PER_PAGE * page_num) as i64,
            search: resolve_search_string(search),
        })
        .await
    }
}

impl CurrentDevelopers {
    /// Send the developers page query (and limit to current developers) and wait for a response.
    pub async fn get(
        page_num: u32,
        search: Option<String>,
    ) -> Result<<Self as GraphQLQuery>::ResponseData, TelescopeError> {
        send_query::<Self>(current_developers::Variables {
            limit: PER_PAGE as i64,
            offset: (PER_PAGE * page_num) as i64,
            search: resolve_search_string(search),
            now: Utc::today().naive_utc(),
        })
        .await
    }
}
