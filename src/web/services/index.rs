//! Module for serving the RCOS homepage.

use actix_web::HttpRequest;
use crate::templates::{
    Template,
    homepage,
    page
};
use crate::error::TelescopeError;
use crate::web::api::rcos::{self, landing_page_stats::{LandingPageStatistics, LandingPageStatsVars}};
use actix_web::client::Client;

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Get the statistics.
    let client: Client = rcos::make_api_client(None);
    let stats = rcos::send_query::<LandingPageStatistics>(&client, LandingPageStatsVars)
        .await?;

    // Make the homepage template as the content of the landing page.
    let content: Template = homepage::new(
        stats.current_semester().unwrap_or("(Unknown Semester)".to_string()),
        stats.current_projects().unwrap_or(-1),
        stats.total_projects().unwrap_or(-1),
        stats.current_students().unwrap_or(-1),
        stats.total_students().unwrap_or(-1),
    );

    // Return a page with the homepage content.
    return page::of(req.path(), "RCOS", &content);
}
