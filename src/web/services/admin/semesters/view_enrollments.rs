//! Semester record creation.

use actix_web::http::header::{
    self as header, ContentDisposition, DispositionParam, DispositionType,
};
use actix_web::web::{self as aweb, Path, Query, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse};
use csv::WriterBuilder;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;

use crate::api::rcos::semesters::get_by_id::semester::SemesterSemestersByPk;
use crate::api::rcos::semesters::get_by_id::Semester;
use crate::api::rcos::users::enrollments::enrollments_lookup::EnrollmentsLookup;
use crate::api::rcos::users::enrollments::user_enrollment_lookup::UserEnrollmentLookup;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::pagination::PaginationInfo;
use crate::templates::Template;
use crate::web::services::admin::semesters::PER_PAGE;
use crate::web::services::auth::identity::Identity;

const TEMPLATE_PATH: &'static str = "admin/semesters/enrollments";
/// The query parameters passed to the developers page indicating pagination
/// data and any filters.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EnrollmentPageQuery {
    /// Filter for users if their first name, last name, or RCS ID contains
    /// this string case independently (via ILIKE).
    pub search: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Enrollments {
    pub semester_id: String,
    pub project_id: String,
    pub is_project_lead: String,
    pub is_coordinator: String,
    pub credits: String,
    pub is_for_pay: String,
    pub mid_year_grade: String,
    pub final_grade: String,
    pub created_at: String,
    pub user_id: String,
}

pub fn register_services(conf: &mut ServiceConfig) {
    // Route with or without the page number to the developers_page handler
    conf.route(
        "/semesters/enrollments/{semester_id}",
        aweb::get().to(enrollments_page),
    )
    .route(
        "/semesters/enrollments/{semester_id}/{page}",
        aweb::get().to(enrollments_page_index),
    );
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

// download page for enrollments csv file.
// When user access to this page, a csv file will be created and written at /tmp/.
#[get("/download/enrollments/{semester_id}")]
pub async fn export_to_csv(
    Path(semester_id): Path<String>,
) -> Result<HttpResponse, TelescopeError> {
    let query_response = EnrollmentsLookup::get(semester_id.clone()).await?;
    let mut buffer = Vec::new();
    // scope to ensure writer is dropped after its done, so we can use the buffer
    {
        let mut wtr = WriterBuilder::new().from_writer(&mut buffer);
        let api_data = query_response.enrollments;
        wtr.serialize(api_data).map_err(|e| {
            TelescopeError::ise(format!(
                "There was an issue writing the data to CSV: {:?}",
                e
            ))
        })?;
        wtr.flush().map_err(|e| {
            TelescopeError::ise(format!(
                "There was an issue finalizing the CSV file: {:?}",
                e
            ))
        })?;
    }
    let resp = HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/csv")
        .set_header(
            header::CONTENT_DISPOSITION,
            ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![DispositionParam::Filename(format!(
                    "enrollments-{}.csv",
                    semester_id
                ))],
            },
        )
        .body(buffer);
    Ok(resp)
}

pub async fn enrollments_page_index(
    req: HttpRequest,
    identity: Identity,
    Path((semester_id, page)): Path<(String, u32)>,
    Query(query): Query<EnrollmentPageQuery>,
) -> Result<Page, TelescopeError> {
    // Resolve the page number from the request
    let mut page_num = page;
    if page_num >= 1 {
        page_num -= 1;
    } else {
        page_num = 0;
    }

    // Get the API data by sending one of the enrollment page queries.
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
    let prefix = "/admin/semesters/enrollments/".to_owned() + &semester_id + "/";
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "pagination": get_page_numbers(&api_data, page_num as u64 + 1),
        "title": semester.title,
        "data": enrollment_data,
        "id": semester_id,
        "identity": viewer,
        "prefix": prefix,
        "preserved_query_string": req.query_string(),
    });
    return template.in_page(&req, "Enrollments").await;
}

pub async fn enrollments_page(
    req: HttpRequest,
    identity: Identity,
    Path(semester_id): Path<String>,
    Query(query): Query<EnrollmentPageQuery>,
) -> Result<Page, TelescopeError> {
    // Get the API data by sending one of the enrollment page queries.
    let semester = Semester::get_by_id(semester_id.clone())
        .await?
        .unwrap_or_else(|| SemesterSemestersByPk {
            semester_id: semester_id.clone(),
            title: "".to_string(),
            start_date: Utc::today().naive_utc(),
            end_date: Utc::today().naive_utc(),
        });
    let query_response =
        UserEnrollmentLookup::get_by_id(0, query.search.clone(), semester_id.clone()).await?;
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
    let prefix = "/admin/semesters/enrollments/".to_owned() + &semester_id + "/";
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "title": semester.title,
        "pagination": get_page_numbers(&api_data, 1),
        "data": enrollment_data,
        "id": semester_id,
        "identity": viewer,
        "prefix": prefix,
        "preserved_query_string": req.query_string(),
    });
    return template.in_page(&req, "Enrollments").await;
}
