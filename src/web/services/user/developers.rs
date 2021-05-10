//! Developers page services

use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::Query;

/// The path to the developers page template from the templates directory.
const TEMPLATE_PATH: &'static str = "user/developers";

/// The query parameters passed to the developers page indicating pagination
/// data and any filters.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DevelopersPageQuery {
    /// What page number to go to. Default to 0.
    #[serde(default)]
    pub page: u32,

    /// Filter for users if their first name, last name, or username contains
    /// this string case independently (via ILIKE).
    pub search: Option<String>,

    /// Should the results include previous members?
    #[serde(default)]
    pub include_old: bool,
}

/// The developer catalogue. This page displays all of the users in the
/// RCOS database.
#[get("/developers")]
pub async fn developers_page(
    identity: Identity,
    Query(query): Query<DevelopersPageQuery>,
) -> Result<Template, TelescopeError> {
    // Explicitly return not implemented to avoid exposing unfinished page.
    return Err(TelescopeError::NotImplemented);

    /*
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

    // Send the query and wait for a response.
    let query_response: DevelopersResponse = send_query::<Developers>(variables).await?.simplify();

    Err(TelescopeError::NotImplemented)

     */
}
