//! RCOS API query to get list of developers to display on the developers page.

// Aliases necessary for generated module's namespace.
use super::UserAccountType as user_account;
use super::UserRole as user_role;

/// Type representing GraphQL query to get a list of users and their
/// account associations.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/users/developers.graphql",
    response_derives = "Debug"
)]
pub struct Developers;

use developers::Variables;
use developers::users_order_by;
use regex::Regex;
use std::borrow::Cow;

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
    pub fn make_variables(limit: u32, offset: u32, search: Option<String>, order_by: users_order_by) -> Variables {
        Variables {
            limit: limit as i64,
            offset: offset as i64,
            // Search string should default to matching any user.
            search: search
                // Escape the search string and surround it in `%`s to match on any sequence.
                .map(|s| format!("%{}%", escape_search_string(s.as_str())))
                // Default to match any user on no search string.
                .unwrap_or("%".into()),
            // In practice, we usually only have one order-by parameter.
            order_by: vec![order_by]
        }
    }
}

impl Default for users_order_by {
    fn default() -> Self {
        users_order_by {
            bonus_attendances_aggregate: None,
            cohort: None,
            created_at: None,
            first_name: None,
            last_name: None,
            username: None,
            enrollments_aggregate: None,
            final_grade_appeals_aggregate: None,
            meeting_attendances_aggregate: None,
            meetings_aggregate: None,
            preferred_name: None,
            project_pitches_aggregate: None,
            project_pitches_by_reviewer_username_aggregate: None,
            project_presentation_grades_aggregate: None,
            role: None,
            small_group_mentors_aggregate: None,
            status_update_submissions_aggregate: None,
            status_update_submissions_by_grader_username_aggregate: None,
            timezone: None,
            user_accounts_aggregate: None,
            workshop_proposals_aggregate: None,
            workshop_proposals_by_reviewer_username_aggregate: None
        }
    }
}
