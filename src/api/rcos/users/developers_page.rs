//! RCOS API query to get list of developers to display on the developers page.

use crate::api::rcos::prelude::*;
use regex::Regex;
use std::borrow::Cow;
use chrono::Utc;

/// The query returns 20 developers per page.
const PER_PAGE: u32 = 20;

/// Type representing GraphQL query to get a list of users and their
/// account associations.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/developers.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Developers;

use developers::{ResponseData, Variables};

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

impl Developers {
    /// Create the variables object to pass to the GraphQL query.
    ///
    /// ## Parameters:
    /// - `limit`: The number of users to return.
    /// - `offset`: The offset into the user data.
    /// - `search`: Case insensitive string to match against user's first name,
    ///     last name, or username. This gets escaped before being sent.
    /// - `order_by`: How to order the users requested.
    fn make_variables(
        page_num: u32,
        search: Option<String>,
        include_old: bool,
    ) -> Variables {
        Variables {
            limit: PER_PAGE as i64,
            offset: PER_PAGE*page_num as i64,
            // Search string should default to matching any user.
            search: search
                // Escape the search string and surround it in `%`s to match on any sequence.
                .map(|s| format!("%{}%", escape_search_string(s.as_str())))
                // Default to match any user on no search string.
                .unwrap_or("%".into()),
            include_old,
            now: Utc::today().naive_utc()
        }
    }

    pub async fn get(page_num: u32, search: Option<String>, include_old: bool) ->
}