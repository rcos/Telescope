//! Services for the semester records management page.

use actix_web::web as aweb;
use actix_web::web::{Path, ServiceConfig};
use actix_web::HttpRequest;
use regex::Regex;

use crate::api::rcos::semesters::get::{Semesters, PER_PAGE};
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::pagination::PaginationInfo;
use crate::templates::Template;

mod create;
mod edit;
mod view_enrollments;
mod view_projects;

/// Register semester services.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(create::new)
        .service(create::submit_new)
        .service(edit::edit)
        .service(edit::submit_edit)
        .route("/semesters", aweb::get().to(index))
        .route("/semesters/{page}", aweb::get().to(index));
}

/// Page to display previous semesters and allow edits.
async fn index(req: HttpRequest, page_num: Option<Path<u32>>) -> Result<Page, TelescopeError> {
    // Resolve the page number. Default to Page 1.
    let page_num: u32 = page_num.map(|path| path.0).unwrap_or(1);

    // Send the API query to get semester data.
    let semester_data = Semesters::get(page_num - 1).await?;

    // Extract the semester count if available.
    let semester_count = semester_data
        .semester_count()
        .ok_or(TelescopeError::ise("Semester count not returned by API"))?
        as u64;

    // Render template and send back to user.
    let mut template = Template::new("admin/semesters/index");
    template.fields = json!({
        "pagination": PaginationInfo::new(semester_count, PER_PAGE as u64, page_num as u64),
        "data": semester_data
    });
    return template.in_page(&req, "Semester Records").await;
}

lazy_static! {
    static ref SEMESTER_ID_REGEX: Regex = Regex::new(r"^[[:digit:]]{6}$").expect("Bad Regex");
}

/// Check if a semester ID is properly formatted (6 digit form) via regex.
fn semester_id_valid(id: &str) -> bool {
    SEMESTER_ID_REGEX.is_match(id)
}
