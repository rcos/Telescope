//! Developers page services

use crate::templates::Template;
use crate::error::TelescopeError;
use actix_web::web::Query;
use crate::web::services::auth::identity::Identity;

/// The default value for the number of users per page.
fn twenty() -> u64 { 20u64 }

/// The query parameters passed to the developers page indicating pagination
/// data and any filters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevelopersPageQuery {
    /// What page number to go to. Default to 0.
    #[serde(rename = "p")]
    #[serde(default)]
    pub page: u64,

    /// How many per page. Default to 20.
    #[serde(rename = "l")]
    #[serde(default = "twenty")]
    pub per_page: u64,

    /// Filter for users if their first name, last name, or username contains
    /// this string case independently (via ILIKE).
    #[serde(rename = "q")]
    pub search: Option<String>,

    /// Filter the semester in which the user was enrolled. This should be a semester ID.
    #[serde(rename = "s")]
    pub filter_semester: Option<String>,

    /// Order the developers by a given field.
    #[serde(rename = "b")]
    pub order_by: Option<String>,

    /// Ascending or descending order.
    #[serde(rename = "o")]
    pub order: Option<String>
}

impl Default for DevelopersPageQuery {
    fn default() -> Self {
        DevelopersPageQuery {
            page: 0,
            per_page: twenty(),
            search: None,
            filter_semester: None,
            order_by: None,
            order: None
        }
    }
}

/// The developer catalogue. This page displays all of the users in the
/// RCOS database.
#[get("/developers")]
pub async fn developers_page(identity: Identity, Query(query): Query<DevelopersPageQuery>) -> Result<Template, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
