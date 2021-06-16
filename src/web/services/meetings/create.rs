//! Creation form and services for meetings.
//!
//! The meeting creation flow is to first direct the user to pick a host,
//! or specify no host. This gets its own page, since it involves searching through
//! all users. Once the meeting creator has made a decision, they are directed to a form
//! to finish meeting creation.

use crate::api::rcos::meetings::authorization_for::AuthorizationFor;
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::HttpResponse;
use actix_web::web::{ServiceConfig, Query};
use futures::future::LocalBoxFuture;
use crate::web::middlewares::authorization::{AuthorizationResult, Authorization};
use actix_web::web as aweb;
use actix_web::guard;
use crate::templates::Template;
use crate::api::rcos::meetings::creation::host_selection::HostSelection;
use actix_web::HttpRequest;

/// Authorization function for meeting creation.
fn meeting_creation_authorization(username: String) -> LocalBoxFuture<'static, AuthorizationResult> {
    Box::pin(async move {
        // Get the meeting authorization
        AuthorizationFor::get(Some(username))
            .await?
            .can_create_meetings()
            // On true, Ok(())
            .then(|| ())
            // Otherwise forbidden
            .ok_or(TelescopeError::Forbidden)
    })
}

/// Register meeting creation services.
fn register(config: &mut ServiceConfig) {
    // Create meeting creation auth middleware.
    let authorization = Authorization::new(meeting_creation_authorization);

    config.service(aweb::scope("/meeting/create")
        .wrap(authorization)
        .service(host_selection_page));
}

/// Query on the host selection page.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct HostSelectionQuery {
    search: String
}

/// Page to select a host for a meeting creation.
/// Authorized to meeting creation perms.
#[get("/host_selection")]
async fn host_selection_page(req: HttpRequest, query: Option<Query<HostSelectionQuery>>) -> Result<Template, TelescopeError> {
    // Extract the query parameter.
    let search: Option<String> = query.map(|q| q.search.clone());
    // Query the RCOS API for host selection data.
    let data = HostSelection::get(search.clone()).await?;

    // Make and return a template.
    Template::new("meetings/creation/host_selection")
        .field("search", search)
        .field("data", data)
        .render_into_page(&req, "Select Host")
        .await
}

/// Endpoint to create a meeting.
pub async fn create_meeting() -> Result<FormTemplate, TelescopeError> {
    return Err(TelescopeError::NotImplemented);
}

/// Endpoint to submit a meeting creation.
#[post("/meeting/create")]
pub async fn submit_new_meeting(
    auth: AuthenticationCookie,
) -> Result<HttpResponse, TelescopeError> {
    Err(TelescopeError::NotImplemented)
}
