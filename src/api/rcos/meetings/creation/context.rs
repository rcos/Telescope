//! GraphQL query to get context for meeting creation.

use crate::api::rcos::send_json_query;
use crate::error::TelescopeError;
use chrono::Utc;
use serde_json::Value;

/// The GraphQL query.
const QUERY_STRING: &'static str =
    include_str!("../../../../../graphql/rcos/meetings/creation/context.graphql");

/// Get the meeting creation context. This is done in JSON format because the typed version of
/// the query variables is bulky and difficult to work with. If there are issues doing this in JSON
/// it may be converted to a strongly typed implementation in the future.
///
/// For meeting edits, semesters may be manually included by ID. otherwise, only ongoing and
/// future semesters will be included.
pub async fn get_context(host_username: Option<String>, include_semesters: Vec<String>) -> Result<Value, TelescopeError> {
    // Make query variables.
    let mut variables: Value = json!({
        // Use an empty or single item list as a work around for non-nullable types.
        "host_username": host_username.clone().map(|username| vec![username]).unwrap_or(Vec::new()),
        "semester_filter": {
            "_or": [
                { "end_date": { "_gte": Utc::today().naive_utc() }},
                { "semester_id": {"_in": include_semesters }}
            ]
        }
    });

    // Add the extra clause to the filter variable if there is a host.
    // This will filter to semesters the host is enrolled in.
    if let Some(username) = host_username {
        variables["semester_filter"]["enrollments"]["username"]["_eq"] = json!(username);
    }

    // Send the query and await the result.
    send_json_query("CreationContext", QUERY_STRING, variables).await
}
