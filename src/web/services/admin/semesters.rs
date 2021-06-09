//! Services for the semester records management page.

use actix_web::web::{ServiceConfig, Path};
use crate::templates::Template;
use crate::error::TelescopeError;
use actix_web::web as aweb;
use crate::api::rcos::semesters::get::{
    Semesters,
    PER_PAGE
};
use crate::templates::pagination::PaginationInfo;
use actix_web::HttpRequest;

/// Register semester services.
pub fn register(config: &mut ServiceConfig) {
    config
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
