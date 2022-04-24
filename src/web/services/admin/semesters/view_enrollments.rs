//! Semester record creation.

use std::fs::File;

use actix_web::web::{self as aweb, Path, Query, ServiceConfig};
use actix_web::HttpRequest;
use actix_files::NamedFile;
use serde::Deserialize;
use serde::ser::Error;
use serde_json::Value;
use csv::WriterBuilder;


use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::api::rcos::users::enrollments::user_enrollment_lookup::UserEnrollmentLookup;
use crate::api::rcos::users::enrollments::enrollments_lookup::EnrollmentsLookup;
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

#[derive(Debug,Serialize, Deserialize, Clone)]
// Record struct stores csv data for a single semester.
pub struct Record{
    pub semester_id: String,
    pub project_id: u32,
    pub is_project_lead: bool,
    pub is_coordinator: bool,
    pub credits: u32,
    pub is_for_pay: bool,
    pub mid_year_grade: f32,
    pub final_grade: f32,
    pub created_at : DateTime<Utc>,
    pub user_id: Uuid,
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
    let mut wtr = WriterBuilder::new().from_path(path.clone()).map_err(|err|{
        error!("Error creating csv file: {}", err);
        err
    });

    if let Err(err)= wtr{
        return Err(TelescopeError::PageNotFound);
    }
    let api_data =  serde_json::to_vec(&query_response);
    wtr.unwrap().write_record(&[api_data.as_ref().unwrap()]).map_err(|err|{
        error!("Error writing to csv file: {}", err);
        err
    });
    if let Err(err)= api_data{
        return Err(TelescopeError::PageNotFound);
    }
    let file = actix_files::NamedFile::open(path).unwrap();
    Ok(file)
    //return Err(TelescopeError::CsrfTokenMismatch);
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
    println!("{:?}", semester_id.clone());
    println!("{:?}", page_num.clone());
   println!("{:?}", query.search.clone());
   let query_response = UserEnrollmentLookup::get_by_id(page_num, query.search.clone(), semester_id.clone()).await?;
   let enrollments = query_response.enrollments.clone();
   println!("enrollments: {:?}", enrollments);
   let enrollment_data = serde_json::to_value(enrollments).unwrap();
   let api_data =  serde_json::to_value(query_response).unwrap();
   println!("{:?}", api_data.get("user_count"));



    //let users = query_response.enrollments.get(0).unwrap().user;
    // Get the viewers user ID
    let viewer: Option<Uuid> = identity.get_user_id().await?;
    let prefix = "/admin/semesters/enrollments/".to_owned()+&semester_id+"/";
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "pagination": get_page_numbers(&api_data, page_num as u64 + 1),
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
    println!("{:?}", semester_id.clone());
   println!("{:?}", query.search.clone());
   let query_response = UserEnrollmentLookup::get_by_id(0, query.search.clone(), semester_id.clone()).await?;
   let enrollments = query_response.enrollments.clone();
   println!("enrollments: {:?}", enrollments);
   let enrollment_data = serde_json::to_value(enrollments).unwrap();
   let api_data =  serde_json::to_value(query_response).unwrap();


    //let users = query_response.enrollments.get(0).unwrap().user;
    // Get the viewers user ID
    let viewer: Option<Uuid> = identity.get_user_id().await?;
    let prefix = "/admin/semesters/enrollments/".to_owned()+&semester_id+"/";
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "pagination": get_page_numbers(&api_data, 1),
        "data": enrollment_data,
        "id": semester_id,
        "identity": viewer,
        "prefix": prefix,
        "preserved_query_string": req.query_string(),
    });
    return template.in_page(&req, "Enrollments").await;

}