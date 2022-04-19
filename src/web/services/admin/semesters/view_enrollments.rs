//! Semester record creation.

use actix_web::web::{self as aweb, Path, Query, ServiceConfig};
use actix_web::HttpRequest;
use serde_json::Value;
use uuid::Uuid;

use crate::api::rcos::users::enrollments::enrollments_lookup::EnrollmentsLookup;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::pagination::PaginationInfo;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use crate::web::services::admin::semesters::PER_PAGE;

const TEMPLATE_PATH: &'static str = "admin/view_enrollments";
/// The query parameters passed to the developers page indicating pagination
/// data and any filters.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EnrollmentPageQuery {
    /// Filter for users if their first name, last name, or RCS ID contains
    /// this string case independently (via ILIKE).
    pub search: Option<String>,

}


pub fn register_services(conf: &mut ServiceConfig) {
    // Route with or without the page number to the developers_page handler
    conf.route("/admin/view_enrollments/{semester_id}", aweb::get().to(enrollments_page))
        .route("/admin/view_enrollments/{semester_id}/{page}", aweb::get().to(enrollments_page));
}

// Create an template for enrollment views.
 fn view_enrollment() -> Template{
    Template::new("admin/semesters/view_enrollments")
}

/// Try to get the pagination bar to use based on the api data.
/// Panics if `current_page` is 0.
fn get_page_numbers(api_response: &Value, current_page: u64) -> Option<PaginationInfo> {
    api_response
        // Check for the JSON field user_count
        .get("user_count")?
        // With field aggregate
        .get("aggregate")?
        // With field count
        .get("count")?
        // As an unsigned integer
        .as_u64()
        // Convert to pagination info
        .and_then(|count| PaginationInfo::new(count, PER_PAGE as u64, current_page))
}


pub async fn enrollments_page(    req: HttpRequest,
    identity: Identity,
    semester: Option<Path<u32>>,
    page: Option<Path<u32>>,
    Query(query): Query<EnrollmentPageQuery>,
) -> Result<Page, TelescopeError>{
    // Resolve the page number from the request
    let page_num: u32 = page
        // Extract from path if available.
        .map(|page_path| page_path.0)
        // Filter and subtract 1, since the page numbers in the UI index from 1.
        // Filter first since subtracting first could result in underflow.
        .filter(|p| *p >= 1)
        .map(|p| p - 1)
        // Otherwise default to 0
        .unwrap_or(0);
    let semester_id : String = semester.unwrap().to_string();
    // Get the API data by sending one of the enrollment page queries.
    let query_response = EnrollmentsLookup::get_by_id(semester_id.clone()).await?;
    let api_data = serde_json::to_value(query_response).unwrap();
    //let users = query_response.enrollments.get(0).unwrap().user;
    // Get the viewers user ID
    let viewer: Option<Uuid> = identity.get_user_id().await?;

    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "pagination": get_page_numbers(&api_data, page_num as u64 + 1),
        "data": api_data,
        "query": query,
        "identity": viewer,
        "preserved_query_string": req.query_string()
    });
    return template.in_page(&req, "Enrollment").await;

}