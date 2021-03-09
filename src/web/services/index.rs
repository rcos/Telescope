//! Module for serving the RCOS homepage.

use crate::error::TelescopeError;
use crate::templates::{homepage, page, Template};
use crate::web::api::rcos::{
    send_query,
    landing_page_stats::{LandingPageStatistics, LandingPageStatsVars},
};
use actix_web::client::Client;
use actix_web::HttpRequest;
use crate::web::services::auth::identity::Identity;

/// Service that serves the telescope homepage.
#[get("/")]
pub async fn index(identity: Identity, req: HttpRequest) -> Result<Template, TelescopeError> {
    // Determine the subject (if they exist) making the stats request.
    let subject: Option<String> = identity.get_rcos_username().await?;
    // Get the statistics.
    let stats = send_query::<LandingPageStatistics>(subject, LandingPageStatsVars).await?;

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
