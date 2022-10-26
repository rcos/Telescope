//! Creation form and services for projects.

use crate::api::rcos::projects::authorization_for::UserProjectAuthorization;
use crate::api::rcos::projects::create::CreateProject;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use crate::web::services::projects::make_projects_auth_middleware;
use actix_web::http::header::LOCATION;
use actix_web::web as aweb;
use actix_web::web::{Form, Query, ServiceConfig};
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde_json::Value;
use uuid::Uuid;


/// The handlebars template to create a project
const CREATION_TEMPLATE: &'static str = "projects/create";

/// Register project creation services.
pub fn register(config: &mut ServiceConfig) {
    // Create project creation auth middleware.
    let authorization =
        make_projects_auth_middleware(&UserProjectAuthorization::can_create);

    config.service(
        aweb::scope("/project")
            .wrap(authorization)
            .service(finish)
            .service(submit_project),
    );
}


/// Query on finish project page.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct FinishQuery {
    host: Uuid,
}

/// Create an empty instance of the form to finish project creation.
async fn finish_form(host: Option<Uuid>) -> Result<Template, TelescopeError> {
    // Query RCOS API for project creation context.
    // Create form.
    let form = Template::new(CREATION_TEMPLATE);

    return Ok(form);
}

/// Endpoint to finish project creation.
#[get("/finish")]
async fn finish(
    req: HttpRequest,
    query: Option<Query<FinishQuery>>,
) -> Result<Page, TelescopeError> {
    // Extract query parameter.
    let host = query.map(|q| q.host);
    // Return form in page.
    finish_form(host)
        .await?
        .in_page(&req, "Create Project")
        .await
}

/// Form submitted by users to create project.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinishForm {
        // $title: String,
        // $stack: _varchar = "",
        // $repository_urls: _url = "",
        // $homepage_url: String = "",
        // $description: String = "",
        // $cover_image_url: String = "") {
    /// Selected semester ID.

    pub title: String,
    pub stack: Vec<String>,
    pub repository_urls: Vec<String>,
    pub homepage_url: String,
    pub description: String,
    pub cover_image_url: String,


}

/// Endpoint that users submit project creation forms to.
#[post("/finish")]
async fn submit_project(
    req: HttpRequest,
    query: Option<Query<FinishQuery>>,
    Form(form): Form<FinishForm>,
) -> Result<HttpResponse, TelescopeError> {
    // Resolve host user ID.
    let host = query.map(|q| q.host.clone());

    // Create a form instance to send back to the user if the one they submitted was invalid.
    let mut return_form: Template = finish_form(host.clone()).await?;
    // Add previously selected fields to the form.
    return_form["selections"] = json!(&form);

    // Validate form fields.
    // Start by destructuring form:

    let FinishForm{
        title,
        stack,
        repository_urls,
        homepage_url,
        description,
        cover_image_url,
    } = form;


    // The title should be null (Option::None) if it is all whitespace or empty.
    // If it is, we don't bother user for this -- they can change the title later and
    // they know if they put in all whitespace. This also decreases form resubmission
    // and template complexity.
    let title: Option<String> = (!title.trim().is_empty()).then(|| title);
    return_form["selections"]["title"] = json!(&title);


    // Check for errors and return form if necessary.
    if return_form["issues"] != json!(null) {
        let page = return_form.in_page(&req, "Create Project").await?;
        return Err(TelescopeError::InvalidForm(page));
    }


    // The rest of the fields are managed pretty tersely in the API call and do not need validation
    // or feedback.
    let created_project_id: i64 = CreateProject::execute(
        title,
        Some(stack),
        Some(repository_urls),
        Some(homepage_url),
        Some(description),
        Some(cover_image_url),
    )
    .await?
    .ok_or(TelescopeError::ise(
        "Project creation call did not return ID.",
    ))?;

    // Redirect the user to the page for the project they created.
    return Ok(HttpResponse::Found()
        .header(LOCATION, format!("/project/{}", created_project_id))
        .finish());
}

/// Get the start and end dates of a selected semester object from the project creation context.
pub fn get_semester_bounds(selected_semester: &Value) -> (NaiveDate, NaiveDate) {
    let semester_start = selected_semester["start_date"]
        .as_str()
        .and_then(|string| string.parse::<NaiveDate>().ok())
        .expect("Semester from context has good start date.");

    let semester_end = selected_semester["end_date"]
        .as_str()
        .and_then(|string| string.parse::<NaiveDate>().ok())
        .expect("Semester from context has good end date.");

    return (semester_start, semester_end);
}
