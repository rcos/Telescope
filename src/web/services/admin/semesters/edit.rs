//! Semester Edit services.

use actix_web::{
    web::Path,
    HttpResponse
};
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::api::rcos::semesters::get_by_id::{
    Semester,
    semester::SemesterSemestersByPk
};
use chrono::NaiveDate;
use actix_web::web::Form;
use crate::api::rcos::semesters::mutations::edit::EditSemester;
use actix_web::http::header::LOCATION;

/// Make the form template for the semester edits.
fn make_edit_form(id: String, title: String, start: NaiveDate, end: NaiveDate) -> FormTemplate {
    let mut form = FormTemplate::new("admin/semesters/forms/edit", "Edit Semester");

    form.template = json!({
        "id": id,
        "title": {"value": title},
        "start": {"value": start},
        "end": {"value": end}
    });

    return form;
}

/// The form submitted for semester edits.
#[derive(Serialize, Deserialize, Debug)]
pub struct SemesterEdits {
    title: String,
    start: NaiveDate,
    end: NaiveDate
}

/// Service to display the semester edit form.
#[get("/semesters/edit/{semester_id}")]
pub async fn edit(Path(semester_id): Path<String>) -> Result<FormTemplate, TelescopeError> {
    // First lookup the semester.
    let semester_data = Semester::get_by_id(semester_id).await?;

    // Make sure it exists.
    if semester_data.is_none() {
        return Err(TelescopeError::resource_not_found(
            "Semester Not Found",
            "Could not find a semester by this ID."));
    }

    // It does, we can unwrap it and deconstruct it.
    let SemesterSemestersByPk { semester_id, title, start_date, end_date } = semester_data.unwrap();
    // Build and return the form with it.
    return Ok(make_edit_form(semester_id, title, start_date, end_date));
}

/// Service to receive semester edits.
#[post("/semesters/edit/{semester_id}")]
pub async fn submit_edit(
    Path(semester_id): Path<String>,
    Form(SemesterEdits { title, start, end}): Form<SemesterEdits>
) -> Result<HttpResponse, TelescopeError> {
    // Assume the semester exist. Return an error later if the GraphQL mutation fails.
    // Start by validating the changes.

    // Validate title
    if title.trim().is_empty() {
        let mut return_form: FormTemplate = make_edit_form(semester_id, title, start, end);
        return_form.template["title"]["issue"] = json!("Title cannot be empty.");
        return Err(TelescopeError::invalid_form(&return_form));
    }

    // Validate dates.
    if start >= end {
        let mut return_form: FormTemplate = make_edit_form(semester_id, title, start, end);
        return_form.template["start"]["issue"] = json!("Start date must be before end date.");
        return Err(TelescopeError::invalid_form(&return_form));
    }

    // Data is valid. Execute changes.
    let edited = EditSemester::execute(semester_id, title, start, end)
        .await?;

    // Check if there was a semester for this ID.
    if edited.is_none() {
        return Err(TelescopeError::resource_not_found(
            "Semester not found.",
            "Could not find a semester by this ID."));
    }

    // Edit success! Redirect user.
    Ok(HttpResponse::Found().header(LOCATION, "/admin/semesters").finish())
}
