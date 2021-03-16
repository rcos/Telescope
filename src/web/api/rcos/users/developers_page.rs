//! RCOS API query to get list of developers to display on the developers page.

// Aliases necessary for generated module's namespace.
use super::UserAccountType as user_account;

/// Type representing GraphQL query to get a list of users and their
/// account associations.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/developers.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Developers;

use developers::{
    users_order_by, DevelopersCurrentSemester, DevelopersUsers, DevelopersUsersUserAccounts,
    ResponseData, Variables,
};
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
    /// - `order_by`: How to order the users requested.
    pub fn make_variables(
        limit: u32,
        offset: u32,
        search: Option<String>,
        order_by: users_order_by,
    ) -> Variables {
        Variables {
            limit: limit as i64,
            offset: offset as i64,
            // Search string should default to matching any user.
            search: search
                // Escape the search string and surround it in `%`s to match on any sequence.
                .map(|s| format!("%{}%", escape_search_string(s.as_str())))
                // Default to match any user on no search string.
                .unwrap_or("%".into()),
            order_by,
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
            workshop_proposals_by_reviewer_username_aggregate: None,
        }
    }
}

/// A user thumbnail on the developers page.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DevelopersPageUser {
    /// The user's username
    pub username: String,
    /// The user's first name
    pub first_name: String,
    /// The user's last name
    pub last_name: String,
    /// The platforms the user has linked.
    pub user_accounts: Vec<DevelopersUsersUserAccounts>,
    /// Is the user a coordinator this semester?
    pub is_current_coordinator: bool,
    /// The small group this user mentors, if any.
    pub small_group_id: Option<i64>,
}

impl DevelopersUsers {
    /// Resolve the semester data on a user to be current or not.
    fn resolve(self, current_semester_id: Option<&String>) -> DevelopersPageUser {
        // Destructure self.
        let DevelopersUsers {
            username,
            first_name,
            last_name,
            user_accounts,
            newest_enrollment,
            small_group_mentors,
        } = self;

        // There should be at most one enrollment
        let newest_enrollment = newest_enrollment.first();
        // And at most one small group
        let small_group_mentored: Option<_> = small_group_mentors
            // Get the first small_group_mentors object
            .first()
            // take a reference to the inner small_group object.
            .map(|o| &o.small_group);

        DevelopersPageUser {
            // inherit trivial fields
            username,
            first_name,
            last_name,
            user_accounts,
            // compute remaining fields
            is_current_coordinator: current_semester_id
                // Map the semester if it exists.
                .and_then(|semester_id| {
                    newest_enrollment
                        // Check that the enrollment if for the current semester
                        .filter(|enrollment| enrollment.semester_id == semester_id.as_str())
                        // Map it to the is_coordinator field
                        .map(|enrollment| enrollment.is_coordinator)
                        // Flatten the double option to Option<bool>
                        .flatten()
                })
                // If there is no semester or an internal field is null,
                // the user is not a coordinator.
                .unwrap_or(false),

            small_group_id: current_semester_id
                // Map the current semester if it exists.
                .and_then(|semester_id| {
                    small_group_mentored
                        // Filter for the same semester
                        .filter(|small_group| small_group.semester_id == semester_id.as_str())
                        // Map to the small group id
                        .map(|small_group| small_group.small_group_id)
                }),
        }
    }
}

/// Simplified structure reflecting response data from developers query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevelopersResponse {
    /// The title of the current semester
    pub current_semester_title: Option<String>,
    /// The users to display on the developers page.
    pub users: Vec<DevelopersPageUser>,
}

impl ResponseData {
    /// Extract the current semester from the Developers query.
    fn current_semester(&self) -> Option<&DevelopersCurrentSemester> {
        self.current_semester.first()
    }

    /// Convert this response into a simplified
    pub fn simplify(self) -> DevelopersResponse {
        // Extract the current semester
        let current_semester = self.current_semester();
        // Extract and clone the semester title
        let current_semester_title: Option<String> = current_semester.map(|s| s.title.clone());
        // Extract and clone the semester id
        let current_semester_id: Option<String> = current_semester.map(|s| s.semester_id.clone());

        // Build and return a simplified response
        DevelopersResponse {
            current_semester_title,
            users: self
                .users
                // Convert the user list to an iterator
                .into_iter()
                // and resolve each user's titles for the current semester.
                .map(|u| u.resolve(current_semester_id.as_ref()))
                // Collect into vector.
                .collect(),
        }
    }
}
