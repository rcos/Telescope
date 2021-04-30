//! Module for serving the RCOS homepage.

use crate::api::rcos::{landing_page_stats::LandingPageStatistics, send_query};
use crate::error::TelescopeError;
use crate::templates::Template;
use actix_web::HttpRequest;

/// Path to the Handlebars file from the templates directory.
const TEMPLATE_PATH: &'static str = "index";

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Get the statistics.
    let stats = LandingPageStatistics::get().await?;

    // Make and return a template with the statistics.
    Template::new(TEMPLATE_PATH)
        .field("stats", stats)
        .render_into_page(&req, "RCOS")
        .await
}
