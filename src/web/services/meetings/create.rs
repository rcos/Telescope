//! Creation form and services for meetings.
//!
//! The meeting creation flow is to first direct the user to pick a host,
//! or specify no host. This gets its own page, since it involves searching through
//! all users. Once the meeting creator has made a decision, they are directed to a form
//! to finish meeting creation.

use crate::api::rcos::meetings::authorization_for::AuthorizationFor;
use crate::api::rcos::meetings::creation;
use crate::api::rcos::meetings::creation::host_selection::HostSelection;
use crate::api::rcos::meetings::{MeetingType, ALL_MEETING_TYPES};
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::templates::Template;
use crate::web::middlewares::authorization::{Authorization, AuthorizationResult};
use actix_web::web as aweb;
use actix_web::web::{Form, Query, ServiceConfig};
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use chrono::NaiveDate;
use futures::future::LocalBoxFuture;
use serde_json::Value;

/// Authorization function for meeting creation.
fn meeting_creation_authorization(
    username: String,
) -> LocalBoxFuture<'static, AuthorizationResult> {
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

    config.service(
        aweb::scope("/meeting/create")
            .wrap(authorization)
            .service(host_selection_page)
            .service(finish)
            .service(submit_meeting),
    );
}

/// Query on the host selection page.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct HostSelectionQuery {
    search: String,
}

/// Page to select a host for a meeting creation.
/// Authorized to meeting creation perms.
#[get("/select_host")]
async fn host_selection_page(
    req: HttpRequest,
    query: Option<Query<HostSelectionQuery>>,
) -> Result<Template, TelescopeError> {
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
    host: String,
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
    let context: Value = creation::context::get_context(host).await?;

    // Create the form template to finish meeting creation.
    let mut form: FormTemplate = finish_form();

    // Add context to form.
    form.template = json!({
        "context": context,
        "meeting_types": &ALL_MEETING_TYPES
    });

    // Return form.
    return Ok(form);
}

/// Form submitted by users to create meeting.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct FinishForm {
    /// Selected semester ID.
    semester: String,

    /// What type of meeting is being created.
    kind: MeetingType,

    /// The optional meeting title. Default empty.
    #[serde(default)]
    title: String,

    start_date: NaiveDate,

    /// Cannot be a [`NaiveTime`], since seconds are not included.
    start_time: String,

    end_date: NaiveDate,

    /// Cannot be a [`NaiveTime`], since seconds are not included.
    end_time: String,

    /// The markdown description of the meeting. Default empty.
    #[serde(default)]
    description: String,

    #[serde(default)]
    is_remote: Option<bool>,

    #[serde(default)]
    meeting_url: Option<String>,

    #[serde(default)]
    location: Option<String>,

    #[serde(default)]
    recording_url: Option<String>,

    #[serde(default)]
    external_slides_url: Option<String>,

    #[serde(default)]
    is_draft: Option<bool>,
}

/// Endpoint that users submit meeting creation forms to.
#[post("/finish")]
async fn submit_meeting(
    query: Option<Query<FinishQuery>>,
    Form(form): Form<FinishForm>,
) -> Result<HttpResponse, TelescopeError> {
    // Resolve host username.
    let host: Option<String> = query.map(|q| q.host.clone());

    Err(TelescopeError::NotImplemented)
}
