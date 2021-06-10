//! Services for the semester records management page.

use actix_web::web::{ServiceConfig, Path, Form};
use crate::templates::Template;
use crate::error::TelescopeError;
use actix_web::{web as aweb, HttpResponse};
use crate::api::rcos::semesters::get::{
    Semesters,
    PER_PAGE
};
use crate::templates::pagination::PaginationInfo;
use actix_web::HttpRequest;
use chrono::NaiveDate;
use crate::templates::forms::FormTemplate;
use regex::Regex;
use crate::api::rcos::semesters::mutations::create::CreateSemester;
use actix_web::http::header::LOCATION;

/// Register semester services.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(new)
        .service(submit_new)
        .route("/semesters", aweb::get().to(index))
        .route("/semesters/{page}", aweb::get().to(index));
}

/// Page to display previous semesters and allow edits.
async fn index(req: HttpRequest, page_num: Option<Path<u32>>) -> Result<Template, TelescopeError> {
    // Resolve the page number. Default to Page 1.
    let page_num: u32 = page_num.map(|path| path.0).unwrap_or(1);

    // Send the API query to get semester data.
    let semester_data = Semesters::get(page_num - 1).await?;

    // Extract the semester count if available.
    let semester_count = semester_data.semester_count()
        .ok_or(TelescopeError::ise("Semester count not returned by API"))?
        as u64;

    // Render template and send back to user.
    Template::new("admin/semesters/index")
        .field("pagination", PaginationInfo::new(semester_count, PER_PAGE as u64, page_num as u64))
        .field("data", semester_data)
        // Render in page
        .render_into_page(&req, "Semester Records")
        .await
}

/// Create an empty form template for semester creation.
fn new_semester_form_empty() -> FormTemplate {
    FormTemplate::new("admin/semesters/forms/create", "Create Semester")
}

/// Semester creation.
#[get("/semesters/create")]
async fn new() -> FormTemplate {
    new_semester_form_empty()
}

/// Form fields submitted when creating a semester record.
#[derive(Debug, Deserialize, Serialize)]
struct CreateSemesterForm {
    /// Semester IDs should be 6 digit strings, as used by the RPI registrar.
    id: String,
    title: String,
    start: NaiveDate,
    end: NaiveDate,
}

lazy_static!{
    static ref SEMESTER_ID_REGEX: Regex = Regex::new(r"^[[:digit:]]{6}$").expect("Bad Regex");
}

/// Check if a semester ID is properly formatted (6 digit form) via regex.
fn semester_id_valid(id: &str) -> bool {
    SEMESTER_ID_REGEX.is_match(id)
}

/// Semester creation forms are submitted here.
#[post("/semesters/create")]
async fn submit_new(Form(input): Form<CreateSemesterForm>) -> Result<HttpResponse, TelescopeError> {
    // Destructure form submission
    let CreateSemesterForm {id, title, start, end} = input;

    // Validate ID.
    if !semester_id_valid(&id) {
        // Create the form returned to the user.
        let mut return_form: FormTemplate = new_semester_form_empty();
        return_form.template = json!({
            "id": {
                "value": id,
                "issue": "Malformed ID. Please use the 6 digit format."
            },
            "title": {"value": title},
            "start": {"value": start},
            "end": {"value": end}
        });

        return Err(TelescopeError::invalid_form(&return_form));
    }

    // Validate title.
    if title.trim().is_empty() {
        let mut return_form: FormTemplate = new_semester_form_empty();
        return_form.template = json!({
            "id": {"value": id},
            "title": {"issue": "Title cannot be empty."},
            "start": {"value": start},
            "end": {"value": end}
        });

        return Err(TelescopeError::invalid_form(&return_form));
    }

    // Validate dates.
    if start >= end {
        let mut return_form: FormTemplate = new_semester_form_empty();
        return_form.template = json!({
            "id": {"value": id},
            "title": {"value": title},
            "start": {"value": start, "issue": "Semester cannot end before it starts."},
            "end": {"value": end}
        });

        return Err(TelescopeError::invalid_form(&return_form));
    }

    // Everything is valid -- create the semester.
    CreateSemester::execute(id, title, start, end).await?;

    // Redirect back to semesters page.
    Ok(HttpResponse::Found()
        .header(LOCATION, "/admin/semesters")
        .finish())
}
