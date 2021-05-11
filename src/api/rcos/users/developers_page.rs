//! RCOS API query to get list of developers to display on the developers page.

use crate::api::rcos::prelude::*;
use regex::Regex;
use std::borrow::Cow;
use chrono::Utc;
use graphql_client::GraphQLQuery;
use crate::error::TelescopeError;
use crate::api::rcos::send_query;

/// The query returns 20 developers per page.
const PER_PAGE: u32 = 20;

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

lazy_static! {
    static ref SEARCH_REGEX: Regex = Regex::new(r"[@%\]").unwrap();
}

/// Escape a search string by putting a back-slash before all
/// special characters (`_`, `%`, or `\`).
fn escape_search_string(search: &str) -> Cow<'_, str> {
    // Replace all instances of the matched regex with themself preceded
    // by a back-slash
    SEARCH_REGEX.replace_all(search, "\\$0")
}

/// Escape a search string and format with hasura regular expression characters
/// or produce the default (all-accepting) search string.
fn resolve_search_string(search: Option<String>) -> String {
    search
        // Escape the search string and surround it in `%`s to match on any sequence.
        .map(|s| format!("%{}%", escape_search_string(s.as_str())))
        // Default to match any user on no search string.
        .unwrap_or("%".into())
}

impl AllDevelopers {
    /// Send the query to get all the developers (including old ones) and wait for a response.
    pub async fn get(page_num: u32, search: Option<String>) -> Result<<Self as GraphQLQuery>::ResponseData, TelescopeError> {
        send_query::<Self>(all_developers::Variables {
            limit: PER_PAGE as i64,
            offset: (PER_PAGE * page_num) as i64,
            search: resolve_search_string(search),
            now: Utc::today().naive_utc()
        }).await
    }
}

impl CurrentDevelopers {
    /// Send the developers page query (and limit to current developers) and wait for a response.
    pub async fn get(page_num: u32, search: Option<String>) -> Result<<Self as GraphQLQuery>::ResponseData, TelescopeError> {
        send_query::<Self>(current_developers::Variables {
            limit: PER_PAGE as i64,
            offset: (PER_PAGE * page_num) as i64,
            search: resolve_search_string(search),
            now: Utc::today().naive_utc()
        }).await
    }
}
