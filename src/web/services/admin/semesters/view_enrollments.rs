//! Semester record creation.

use actix_web::web::{self as aweb, Path, Query, ServiceConfig};
use actix_web::HttpRequest;
use actix_web::http::header::{ContentDisposition,DispositionType, DispositionParam};
use actix_files::NamedFile;
use serde::Serialize;
use serde_json::Value;
use csv::WriterBuilder;
use uuid::Uuid;

use crate::api::rcos::users::enrollments::user_enrollment_lookup::UserEnrollmentLookup;
use crate::api::rcos::users::enrollments::enrollments_lookup::EnrollmentsLookup;
use crate::api::rcos::semesters::get_by_id::Semester;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::pagination::PaginationInfo;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use crate::web::services::admin::semesters::PER_PAGE;


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
pub struct Enrollments{
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
    conf.route("/semesters/enrollments/{semester_id}", aweb::get().to(enrollments_page))
        .route("/semesters/enrollments/{semester_id}/{page}", aweb::get().to(enrollments_page_index));
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

#[get("/download/enrollments/{semester_id}")]
pub async fn export_to_csv(Path(semester_id) : Path<String>) -> Result<NamedFile, TelescopeError> {
    let query_response = EnrollmentsLookup::get(semester_id.clone()).await?;
    let  path = "/tmp/".to_owned()+semester_id.as_str().clone()+"_enrollments.csv";
    let mut wtr = WriterBuilder::new().from_path(path.clone()).unwrap();
    let api_data = query_response.enrollments;
    for data in api_data{
        wtr.serialize(Enrollments{
            semester_id: data.semester_id.to_string(),
            project_id: data.project_id.map(|v| v.to_string()).unwrap_or_else(|| "".to_string()),
            is_project_lead: data.is_project_lead.to_string(),
            is_coordinator: data.is_coordinator.to_string(),
            credits: data.credits.to_string(),
            is_for_pay: data.is_for_pay.to_string(),
            mid_year_grade: data.mid_year_grade.map(|v| v.to_string()).unwrap_or_else(|| "".to_string()),
            final_grade: data.final_grade.map(|v| v.to_string()).unwrap_or_else(|| "".to_string()),
            created_at: data.created_at.to_string(),
            user_id: data.user_id.to_string(),
        }).unwrap();
    }
    wtr.flush();

    let file = actix_files::NamedFile::open(path);
    if !file.is_ok() {
        return Err(TelescopeError::PageNotFound);
    }
    let param = DispositionParam::Filename(semester_id+"_enrollments.csv");
    Ok(file.unwrap().set_content_disposition(ContentDisposition{
        disposition: DispositionType::Attachment,
        parameters: vec![param],
    } ))
}


pub async fn enrollments_page_index(req: HttpRequest,
    identity: Identity,
    Path((semester_id, page)): Path<(String, u32)>,
    Query(query): Query<EnrollmentPageQuery>,
) -> Result<Page, TelescopeError>{
    // Resolve the page number from the request
    let mut page_num =  page;
    if page_num >= 1{
        page_num -= 1;
    }else{
        page_num = 0;
    }

    // Get the API data by sending one of the enrollment page queries.
   let semester = Semester::get_by_id(semester_id.clone()).await?;
   let query_response = UserEnrollmentLookup::get_by_id(page_num, query.search.clone(), semester_id.clone()).await?;
   let enrollments = query_response.enrollments.clone();
   let enrollment_data = serde_json::to_value(enrollments).unwrap();
   let api_data =  serde_json::to_value(query_response).unwrap();

    //let users = query_response.enrollments.get(0).unwrap().user;
    // Get the viewers user ID
    let viewer: Option<Uuid> = identity.get_user_id().await?;
    let prefix = "/admin/semesters/enrollments/".to_owned()+&semester_id+"/";
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "pagination": get_page_numbers(&api_data, page_num as u64 + 1),
        "title": semester.unwrap().title,
        "data": enrollment_data,
        "id": semester_id,
        "identity": viewer,
        "prefix": prefix,
        "preserved_query_string": req.query_string(),
    });
    return template.in_page(&req, "Enrollments").await;
}

pub async fn enrollments_page(req: HttpRequest,
    identity: Identity,
    Path(semester_id): Path<String>,
    Query(query): Query<EnrollmentPageQuery>,
) -> Result<Page, TelescopeError>{

    // Get the API data by sending one of the enrollment page queries.
    let semester = Semester::get_by_id(semester_id.clone()).await?;
    let query_response = UserEnrollmentLookup::get_by_id(0, query.search.clone(), semester_id.clone()).await?;
    let enrollments = query_response.enrollments.clone();
    let enrollment_data = serde_json::to_value(enrollments).unwrap();
    let api_data =  serde_json::to_value(query_response).unwrap();


    //let users = query_response.enrollments.get(0).unwrap().user;
    // Get the viewers user ID
    let viewer: Option<Uuid> = identity.get_user_id().await?;
    let prefix = "/admin/semesters/enrollments/".to_owned()+&semester_id+"/";
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "title": semester.unwrap().title,
        "pagination": get_page_numbers(&api_data, 1),
        "data": enrollment_data,
        "id": semester_id,
        "identity": viewer,
        "prefix": prefix,
        "preserved_query_string": req.query_string(),
    });
    return template.in_page(&req, "Enrollments").await;

}