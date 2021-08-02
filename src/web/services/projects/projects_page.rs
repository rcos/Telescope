//! Project page.

use crate::error::TelescopeError;
use actix_web::HttpResponse;
use crate::api::rcos::projects::projects_page::CurrentProjects;
use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::api::rcos::meetings::get::Meetings;
use crate::api::rcos::meetings::MeetingType;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::{Query, ServiceConfig};
use actix_web::HttpRequest;
use chrono::{Date, DateTime, Duration, Local, NaiveDate, TimeZone, Utc};

/// Register the projects page
pub fn register(c: &mut ServiceConfig) -> &mut ServiceConfig {
    c.service(projects_list)
}

/// The path to the template's handlebars file.
const TEMPLATE_PATH: &'static str = "projects/list";

#[get("/projects")]
async fn projects_list(
    req: HttpRequest,
    identity: Identity,
    
) -> Result<Template, TelescopeError> {
    let page=0;
    let search: Option<String> =Some(" ".to_string());
    let projects = CurrentProjects::get(page, search).await?;
    
    return Template::new(TEMPLATE_PATH)
        .field("projects", projects)
        .render_into_page(&req, "RCOS Projects")
        .await;
}