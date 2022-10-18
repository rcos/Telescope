use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::api::rcos::users::enrollments::{
    enrollment_by_ids::{EnrollmentByIds},
};
use crate::api::rcos::projects::projects_page::CurrentProjects;
use crate::templates::Template;
use actix_web::web::Form;
use actix_web::http::header::LOCATION;
use chrono::{DateTime, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde_json::Value;
use actix_web::web::{self as aweb, Path, Query, ServiceConfig, HttpRequest};
use crate::api::rcos::prelude::*;

const ENROLLMENT_EDIT_FORM: &str = "coordinate/enrollments/edit/form";

pub fn register(config: &mut ServiceConfig){
    config.route(
        "/semesters/enrollments/{semester_id}/{user_id}/edit",
        aweb::get().to(edit_page),
                 );
}

pub async fn edit_page(
req: HttpRequest,
Path((semester_id, user_id)): Path<(String, String)>,
) -> Result<Page, TelescopeError>{
    let uid = user_id.parse::<uuid>().ok().unwrap();
    let enrollment_data =  EnrollmentByIds::get(uid, semester_id).await?;
   
    dbg!("{}", &enrollment_data);

    let mut form = Template::new(ENROLLMENT_EDIT_FORM);
    form.fields = json!({
        "data": enrollment_data,
    });

    
    form.in_page(&req, "Edit Enrollment").await
}

pub async fn submit_edits(){

}
