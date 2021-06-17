//! Creation form and services for meetings.
//!
//! The meeting creation flow is to first direct the user to pick a host,
//! or specify no host. This gets its own page, since it involves searching through
//! all users. Once the meeting creator has made a decision, they are directed to a form
//! to finish meeting creation.

use crate::api::rcos::meetings::authorization_for::AuthorizationFor;
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use actix_web::web::{ServiceConfig, Query};
use futures::future::LocalBoxFuture;
use crate::web::middlewares::authorization::{AuthorizationResult, Authorization};
use actix_web::web as aweb;
use crate::templates::Template;
use crate::api::rcos::meetings::creation::host_selection::HostSelection;
use actix_web::HttpRequest;
use crate::api::rcos::meetings::creation::context::CreationContext;

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
pub fn register(config: &mut ServiceConfig) {
    // Create meeting creation auth middleware.
    let authorization = Authorization::new(meeting_creation_authorization);

    config.service(aweb::scope("/meeting/create")
        .wrap(authorization)
        .service(host_selection_page)
        .service(finish));
}

/// Query on the host selection page.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct HostSelectionQuery {
    search: String
}

/// Page to select a host for a meeting creation.
/// Authorized to meeting creation perms.
#[get("/select_host")]
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

/// Query on finish meeting page.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct FinishQuery {
    host: String
}

fn finish_form() -> FormTemplate {
    FormTemplate::new("meetings/creation/forms/finish", "Create Meeting")
}

/// Endpoint to finish meeting creation.
#[get("/finish")]
async fn finish(query: Option<Query<FinishQuery>>) -> Result<FormTemplate, TelescopeError> {
    // Extract query parameter.
    let host: Option<String> = query.map(|q| q.host.clone());
    // Query RCOS API for meeting creation context.
    let context = CreationContext::get(host).await?;
    // Create the form template to finish meeting creation.
    let mut form: FormTemplate = finish_form();

    // Add context to form.
    form.template = json!({
        "context": context,
    });

    // Return form.
    return Ok(form);
}
