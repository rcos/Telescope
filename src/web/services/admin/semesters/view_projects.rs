//! Semester record creation.

use crate::api::rcos::semesters::mutations::create::CreateSemester;
use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::services::admin::semesters::semester_id_valid;
use actix_web::http::header::LOCATION;
use actix_web::{web::Form, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDate;


// Create an template for projects views.
 fn view_projects() -> Template{
    Template::new("admin/semesters/view_proejcts")
}

#[get("/semesters/view_projects/{semester_id}")]
async fn view_enrollment_get(req: HttpRequest) -> impl Responder {
    view_projects().in_page(&req, "View Projects").await
}
