//! Module for serving the RCOS homepage.

use crate::api::rcos::landing_page_stats::LandingPageStatistics;
use crate::error::TelescopeError;
use crate::templates::Template;
use actix_web::HttpRequest;
use crate::templates::page::Page;

/// Path to the Handlebars file from the templates directory.
const TEMPLATE_PATH: &'static str = "index";

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Page, TelescopeError> {
    // Get the statistics.
    let stats = LandingPageStatistics::get().await?;
    // Make and return a template with the statistics.
    let mut template = Template::new(TEMPLATE_PATH);
    template["stats"] = json!(stats);
    return template.in_page(&req, "RCOS").await;
}
