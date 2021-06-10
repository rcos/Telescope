//! Developers page services

use actix_web::web::{self as aweb, Path, Query, ServiceConfig};
use actix_web::HttpRequest;
use serde_json::Value;

use crate::api::rcos::users::developers_page::{AllDevelopers, CurrentDevelopers, PER_PAGE};
use crate::error::TelescopeError;
use crate::templates::pagination::PaginationInfo;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;

/// The path to the developers page template from the templates directory.
const TEMPLATE_PATH: &'static str = "user/developers";

/// Handlebars key for the query info.
const QUERY: &'static str = "query";

/// Handlebars key for the RCOS API data.
const DATA: &'static str = "data";

/// Handlebars key for the viewer's username.
const IDENTITY: &'static str = "identity";

/// Handlebars key for the preserved query string from the request.
/// This should be appended to sub-page links in the template to maintain
/// consistent user ordering.
const PRESERVED_QUERY: &'static str = "preserved_query_string";

/// Handlebars key for the value of the pagination info.
const PAGINATION: &'static str = "pagination";

/// The query parameters passed to the developers page indicating pagination
/// data and any filters.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DevelopersPageQuery {
    /// Filter for users if their first name, last name, or username contains
    /// this string case independently (via ILIKE).
    pub search: Option<String>,

    /// Should the results include previous members?
    #[serde(default)]
    pub include_old: bool,
}

pub fn register_services(conf: &mut ServiceConfig) {
    // Route with or without the page number to the developers_page handler
    conf.route("/developers", aweb::get().to(developers_page))
        .route("/developers/{page}", aweb::get().to(developers_page));
}

/// Try to get the pagination bar to use based on the api data.
/// Panics if `current_page` is 0.
fn get_page_numbers(api_response: &Value, current_page: u64) -> Option<PaginationInfo> {
    api_response
        // Check for the JSON field user_count
        .get("user_count")?
        // With field aggregate
        .get("aggregate")?
        // With field count
        .get("count")?
        // As an unsigned integer
        .as_u64()
        // Convert to pagination info
        .and_then(|count| PaginationInfo::new(count, PER_PAGE as u64, current_page))
}

/// The developer catalogue. This page displays all of the users in the
/// RCOS database.
pub async fn developers_page(
    req: HttpRequest,
    identity: Identity,
    page: Option<Path<u32>>,
    Query(query): Query<DevelopersPageQuery>,
) -> Result<Template, TelescopeError> {
    // Resolve the page number from the request
    let page_num: u32 = page
        // Extract from path if available.
        .map(|page_path| page_path.0)
        // Filter and subtract 1, since the page numbers in the UI index from 1.
        // Filter first since subtracting first could result in underflow.
        .filter(|p| *p >= 1)
        .map(|p| p - 1)
        // Otherwise default to 0
        .unwrap_or(0);

    // Get the API data by sending one of the developer page queries.
    let api_data: Value;
    // Determine which API query to send using the request query.
    if query.include_old {
        // Get all the developers (including ones not active this semester).
        let query_response = AllDevelopers::get(page_num, query.search.clone()).await?;
        // Convert the response into a JSON value.
        // Unwrap because this conversion should never fail.
        api_data = serde_json::to_value(query_response).unwrap();
    } else {
        // Get only the current developers.
        let query_response = CurrentDevelopers::get(page_num, query.search.clone()).await?;
        api_data = serde_json::to_value(query_response).unwrap();
    }

    // Get the viewers username
    let viewer_username: Option<String> = identity.get_rcos_username().await?;

    // Render the developers page template and return it inside a page.
    Template::new(TEMPLATE_PATH)
        // Add 1 to the page number for use in UI.
        .field(PAGINATION, get_page_numbers(&api_data, page_num as u64 + 1))
        .field(DATA, api_data)
        .field(QUERY, query)
        .field(IDENTITY, viewer_username)
        .field(PRESERVED_QUERY, req.query_string())
        .render_into_page(&req, "Developers")
        .await
}
