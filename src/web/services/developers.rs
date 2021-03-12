//! Developers page services

use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::api::rcos::{
    send_query,
    users::developers_page::{
        developers::{order_by, users_order_by, DevelopersUsers},
        Developers,
    },
};
use crate::web::services::auth::identity::Identity;
use actix_web::web::Query;
use crate::web::api::rcos::users::developers_page::DevelopersResponse;

/// The default value for the number of users per page.
fn twenty() -> u32 {
    20
}

/// Which field should users be ordered by.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderByField {
    FirstName,
    LastName,
}

impl Default for OrderByField {
    fn default() -> Self {
        OrderByField::FirstName
    }
}

/// What order to use.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum OrderBy {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl Default for OrderBy {
    fn default() -> Self {
        OrderBy::Ascending
    }
}

impl Into<order_by> for OrderBy {
    fn into(self) -> order_by {
        match self {
            OrderBy::Ascending => order_by::asc,
            OrderBy::Descending => order_by::desc,
        }
    }
}

/// The query parameters passed to the developers page indicating pagination
/// data and any filters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevelopersPageQuery {
    /// What page number to go to. Default to 0.
    #[serde(default)]
    pub page: u32,

    /// How many per page. Default to 20.
    #[serde(default = "twenty")]
    pub per_page: u32,

    /// Filter for users if their first name, last name, or username contains
    /// this string case independently (via ILIKE).
    pub search: Option<String>,

    /// Order the developers by a given field.
    #[serde(default)]
    pub order_by: OrderByField,

    /// Ascending or descending order.
    #[serde(default)]
    pub order: OrderBy,
}

impl Default for DevelopersPageQuery {
    fn default() -> Self {
        DevelopersPageQuery {
            page: 0,
            per_page: twenty(),
            search: None,
            order_by: OrderByField::default(),
            order: OrderBy::default(),
        }
    }
}

impl DevelopersPageQuery {
    /// Convert this structure's order parameters to an order_by query.
    fn order_by_param(&self) -> users_order_by {
        match self.order_by {
            OrderByField::FirstName => users_order_by {
                first_name: Some(self.order.into()),
                ..users_order_by::default()
            },

            OrderByField::LastName => users_order_by {
                last_name: Some(self.order.into()),
                ..users_order_by::default()
            },
        }
    }
}

/// The developer catalogue. This page displays all of the users in the
/// RCOS database.
#[get("/developers")]
pub async fn developers_page(
    identity: Identity,
    Query(query): Query<DevelopersPageQuery>,
) -> Result<Template, TelescopeError> {
    // Extract the number of users to retrieve.
    let limit: u32 = query.per_page;
    // Extract the offset into the user data for the API query.
    let offset: u32 = query.per_page * query.page;
    // Extract the the ordering parameter
    let order_by_param: users_order_by = query.order_by_param();
    // Clone the search text.
    let search_text: Option<String> = query.search.clone();

    // Build the query variables for the GraphQL request.
    let variables = Developers::make_variables(limit, offset, search_text, order_by_param);

    // Extract the user identity for the query subject header (if it exists.)
    let subject: Option<String> = identity.get_rcos_username().await?;
    // Send the query and wait for a response.
    let query_response: DevelopersResponse = send_query::<Developers>(subject, variables).await?.simplify();


    Err(TelescopeError::NotImplemented)
}
