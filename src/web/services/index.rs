//! Module for serving the RCOS homepage.

use crate::api::rcos::landing_page_stats::LandingPageStatistics;
use crate::error::TelescopeError;
use crate::templates::Template;
use actix_web::HttpRequest;
use actix_web::web::Query;

/// Path to the Handlebars file from the templates directory.
const TEMPLATE_PATH: &'static str = "index";

/// If this is set, a notice is shown on the homepage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexQuery {
    pub notice: String,
}

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(req: HttpRequest, params: Option<Query<IndexQuery>>) -> Result<Template, TelescopeError> {
    // Get the statistics.
    let stats = LandingPageStatistics::get().await?;

    // Make and return a template with the statistics.
    Template::new(TEMPLATE_PATH)
        .field("stats", stats)
        .field("notice", params.map(|x| x.notice.to_owned()))
        .render_into_page(&req, "RCOS")
        .await
}
