//! Semester record creation.

use crate::api::rcos::semesters::mutations::create::CreateSemester;
use crate::error::TelescopeError;
use crate::web::services::admin::semesters::semester_id_valid;
use actix_web::http::header::LOCATION;
use actix_web::{web::Form, HttpResponse, HttpRequest, Responder};
use chrono::NaiveDate;
use crate::templates::page::Page;
use crate::templates::Template;

/// Create an empty form template for semester creation.
fn new_semester_form_empty() -> Template {
    Template::new("admin/semesters/forms/create")
}

/// Semester creation.
#[get("/semesters/create")]
pub async fn new(req: HttpRequest) -> impl Responder {
    new_semester_form_empty()
        .in_page(&req, "Create Semester")
        .await
}

/// Form fields submitted when creating a semester record.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSemesterForm {
    /// Semester IDs should be 6 digit strings, as used by the RPI registrar.
    id: String,
    title: String,
    start: NaiveDate,
    end: NaiveDate,
}

/// Semester creation forms are submitted here.
#[post("/semesters/create")]
pub async fn submit_new(
    req: HttpRequest,
    Form(input): Form<CreateSemesterForm>,
) -> Result<HttpResponse, TelescopeError> {
    // Destructure form submission
    let CreateSemesterForm {
        id,
        title,
        start,
        end,
    } = input;

    // Validate ID.
    if !semester_id_valid(&id) {
        // Create the form returned to the user.
        let mut return_form_template: Template = new_semester_form_empty();
        return_form_template.fields = json!({
            "id": {
                "value": id,
                "issue": "Malformed ID. Please use the 6 digit format."
            },
            "title": {"value": title},
            "start": {"value": start},
            "end": {"value": end}
        });

        // Put it in a page.
        let page = return_form_template.in_page(&req, "Create Semester").await?;
        // Return the page.
        return Err(TelescopeError::InvalidForm(page));
    }

    // Validate title.
    if title.trim().is_empty() {
        let mut return_form_template: Template = new_semester_form_empty();
        return_form_template.fields = json!({
            "id": {"value": id},
            "title": {"issue": "Title cannot be empty."},
            "start": {"value": start},
            "end": {"value": end}
        });

        let page = return_form_template.in_page(&req, "Create Semester").await?;
        return Err(TelescopeError::InvalidForm(page));
    }

    // Validate dates.
    if start >= end {
        let mut return_form_template: Template = new_semester_form_empty();
        return_form_template.fields = json!({
            "id": {"value": id},
            "title": {"value": title},
            "start": {"value": start, "issue": "Semester cannot end before it starts."},
            "end": {"value": end}
        });

        let page = return_form_template.in_page(&req, "Create Semester").await?;
        return Err(TelescopeError::InvalidForm(page));
    }

    // Everything is valid -- create the semester.
    CreateSemester::execute(id, title, start, end).await?;

    // Redirect back to semesters page.
    Ok(HttpResponse::Found()
        .header(LOCATION, "/admin/semesters")
        .finish())
}
