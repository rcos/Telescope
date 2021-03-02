//! Homepage template. Holds Handlebars keys for homepage stats.

use crate::templates::Template;
use std::fmt::Display;

/// Path to the Handlebars file from the templates directory.
const TEMPLATE_PATH: &'static str = "index";

/// Handlebars key for the current semester string. The value under this key
/// should be something along the lines of "Fall 2021".
pub const CURRENT_SEMESTER: &'static str = "current_semester";

/// Handlebars key for the number of projects active during the current semester.
pub const ACTIVE_PROJECTS: &'static str = "active_projects";

/// Handlebars key for the total number of projects registered with RCOS.
pub const PROJECT_COUNT: &'static str = "total_projects";

/// Handlebars key for the number of students active in RCOS this semester.
pub const CURRENT_STUDENTS: &'static str = "current_students";

/// Handlebars key for the total number of telescope users.
pub const TOTAL_USERS: &'static str = "total_users";

/// Create a new homepage template with the necessary fields.
pub fn new(
    semester_str: impl Into<String>,
    active_projects: impl Display,
    total_projects: impl Display,
    active_students: impl Display,
    total_users: impl Display,
) -> Template {
    Template::new(TEMPLATE_PATH)
        .field(CURRENT_SEMESTER, semester_str.into())
        .field(ACTIVE_PROJECTS, format!("{}", active_projects))
        .field(PROJECT_COUNT, format!("{}", total_projects))
        .field(CURRENT_STUDENTS, format!("{}", active_students))
        .field(TOTAL_USERS, format!("{}", total_users))
}
