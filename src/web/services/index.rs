//! Module for serving the RCOS homepage.

use crate::error::TelescopeError;
use crate::templates::{homepage, page, Template};
use crate::web::api::rcos::{
    landing_page_stats::{LandingPageStatistics, LandingPageStatsVars},
    send_query,
};
use crate::web::services::auth::identity::Identity;
use actix_web::HttpRequest;

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Get the statistics.
    let stats = send_query::<LandingPageStatistics>(LandingPageStatsVars).await?;

    // Make the homepage template as the content of the landing page.
    let content: Template = homepage::new(
        stats
            .current_semester()
            .unwrap_or("(Unknown Semester)".to_string()),
        stats.current_projects().unwrap_or(-1),
        stats.total_projects().unwrap_or(-1),
        stats.current_students().unwrap_or(-1),
        stats.total_students().unwrap_or(-1),
    );

    // Return a page with the homepage content.
    return page::of(&req, "RCOS", &content).await;
}
