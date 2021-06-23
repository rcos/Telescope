//! GraphQL query to get context for meeting creation.

use crate::api::rcos::send_json_query;
use crate::error::TelescopeError;
use serde_json::Value;
use chrono::Utc;

/// The GraphQL query.
const QUERY_STRING: &'static str = include_str!("../../../../../graphql/rcos/meetings/creation/context.graphql");

/// Get the meeting creation context. This is done in JSON format because the typed version was
/// causing the compiler to stack overflow.
pub async fn get_context(host_username: Option<String>) -> Result<Value, TelescopeError> {
    // Make query variables.
    let mut variables: Value = json!({
        // Use an empty or single item list as a work around for non-nullable types.
        "host_username": host_username.clone().map(|username| vec![username]).unwrap_or(Vec::new()),
        "semester_filter": {
            "end_date": {
                "_gte": Utc::today().naive_utc()
            }
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
