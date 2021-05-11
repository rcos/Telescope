//! Developers page services

use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::{
    Query, Path, ServiceConfig,
    self as aweb
};
use actix_web::HttpRequest;

/// The path to the developers page template from the templates directory.
const TEMPLATE_PATH: &'static str = "user/developers";

/// Handlebars key for the page number.
const PAGE_NUM: &'static str = "page_num";

/// Handlebars key for the query info.
const QUERY: &'static str = "query";

/// Handlebars key for the RCOS API data.
const DATA: &'static str = "data";

/// Handlebars key for the viewer's username.
const IDENTITY: &'static str = "identity";

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
    conf
        .route("/developers", aweb::get().to(developers_page))
        .route("/developers/{page}", aweb::get().to(developers_page));
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
        // Extract from path if available
        .map(|page_path| page_path.0)
        // Otherwise default to 0
        .unwrap_or(0);

    // Send the API query
    let api_data = unimplemented!();
        //Developers::get(page_num, query.search.clone(), query.include_old).await?;

    // Get the viewers username
    let viewer_username: Option<String> = identity.get_rcos_username().await?;

    // Render the developers page template and return it inside a page.
    Template::new(TEMPLATE_PATH)
        .field(PAGE_NUM, page_num)
        .field(DATA, api_data)
        .field(QUERY, query)
        .field(IDENTITY, viewer_username)
        .render_into_page(&req, "Developers")
        .await
}
