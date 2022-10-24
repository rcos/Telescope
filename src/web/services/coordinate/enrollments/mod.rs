use actix_web::web::{self as aweb, Path, Query, ServiceConfig};
use actix_web::HttpRequest;
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use crate::api::rcos::semesters::get_by_id::Semester;
use crate::api::rcos::users::role_lookup::RoleLookup;
use crate::api::rcos::users::UserRole;
use crate::api::rcos::semesters::get_by_id::semester::SemesterSemestersByPk;
use crate::error::TelescopeError;
use crate::api::rcos::users::enrollments::user_enrollment_lookup::UserEnrollmentLookup;
use crate::templates::page::Page;
use crate::templates::pagination::PaginationInfo;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use crate::api::rcos::semesters::get::PER_PAGE;

//mod create;
mod edit;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EnrollmentPageQuery {
    /// Filter for users if their first name, last name, or RCS ID contains
    /// this string case independently (via ILIKE).
    pub search: Option<String>,
}

const TEMPLATE_PATH: &str = "coordinate/enrollments/index";

//Register enrollment management
pub fn register(config: &mut ServiceConfig) {
    edit::register(config);
    config.route(
        "/semesters/enrollments/{semester_id}/{page}",
        aweb::get().to(enrollments_page_index),
    );
    config.route(
        "/semesters/enrollments/{semester_id}",
        aweb::get().to(enrollments_page_index),
    );
}

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

pub async fn enrollments_page_index(
    req: HttpRequest,
    identity: Identity,
    Path((semester_id, page)): Path<(String, u32)>,
    Query(query): Query<EnrollmentPageQuery>,
) -> Result<Page, TelescopeError>{
    let mut page_num = page;
    if page_num >= 1{
        page_num -= 1;
    }
    else{
        page_num = 0;
    }

    let semester = Semester::get_by_id(semester_id.clone())
        .await?
        .unwrap_or_else(|| SemesterSemestersByPk {
            semester_id: semester_id.clone(),
            title: "".to_string(),
            start_date: Utc::today().naive_utc(),
            end_date: Utc::today().naive_utc(),
        });
    let query_response =
        UserEnrollmentLookup::get_by_id(page_num, query.search.clone(), semester_id.clone())
            .await?;
    let enrollments = query_response.enrollments.clone();
    let enrollment_data = serde_json::to_value(enrollments).map_err(|e| {
        TelescopeError::ise(format!(
            "Could not serialize enrollments data to JSON format: {}",
            e
        ))
    })?;
    let api_data = serde_json::to_value(query_response).map_err(|e| {
        TelescopeError::ise(format!(
            "Could not serialize API data to JSON format: {}",
            e
        ))
    })?;

    // Get the viewers user ID
    let viewer: Option<Uuid> = identity.get_user_id().await?;
    let prefix = "/coordinate/semesters/enrollments/".to_owned() + &semester_id + "/";
    let mut template = Template::new(TEMPLATE_PATH);
    let is_not_administrator: bool;
    match viewer {
        Some(x) => {
            let role: UserRole = RoleLookup::get(x)
                .await?
                .expect("Viewer's account does not exist");
            is_not_administrator = !role.is_admin();
        },
        None => is_not_administrator = true, 
    };
    
    template.fields = json!({
        "pagination": get_page_numbers(&api_data, page_num as u64 + 1),
        "title": semester.title,
        "data": enrollment_data,
        "semester_id": semester_id,
        "identity": viewer,
        "prefix": prefix,
        "preserved_query_string": req.query_string(),
        "is_not_admin": is_not_administrator,
    });
    template.in_page(&req, "Enrollments").await
} 
