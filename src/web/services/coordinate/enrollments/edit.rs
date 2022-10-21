use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::api::rcos::users::enrollments::enrollment_by_ids::EnrollmentByIds;
use crate::templates::Template;
use actix_web::web::Form;
use crate::api::rcos::users::enrollments::edit_enrollment::edit_enrollment::Variables;
use crate::api::rcos::users::enrollments::edit_enrollment;
use actix_web::web::{Path, ServiceConfig, HttpRequest};
use crate::api::rcos::prelude::*;

const ENROLLMENT_EDIT_FORM: &str = "coordinate/enrollments/edit/form";

pub fn register(config: &mut ServiceConfig){
    config
        .service(edit_page)
        .service(submit_edits);
}

#[get("/semesters/enrollments/{semester_id}/{user_id}/edit")]
pub async fn edit_page(
req: HttpRequest,
Path((semester_id, user_id)): Path<(String, String)>,
) -> Result<Page, TelescopeError>{
    let uuid = user_id.parse::<uuid>().ok().unwrap();
    let enrollment_data =  EnrollmentByIds::get(uuid, semester_id).await?;
   
    let mut form = Template::new(ENROLLMENT_EDIT_FORM);
    form.fields = json!({
        "data": enrollment_data,
    });

    
    form.in_page(&req, "Edit Enrollment").await
}

#[post("/semesters/enrollments/{semester_id}/{user_id}/edit")]
pub async fn submit_edits(
req: HttpRequest,
Path((semester_id, user_id)): Path<(String, String)>,
Form(form_data): Form<EnrollmentForm>,
) -> Result<Page, TelescopeError>{
    let uid = user_id.parse::<uuid>().ok().unwrap();

    let EnrollmentForm {
        lead,
        coordinator,
        pay,
        project,
        credit,
        mid_grade,
        final_grade,
    } = form_data;
    let uuid = user_id.parse::<uuid>().ok().unwrap();

    let edit_variables = Variables {
        semester_id: semester_id.clone(),
        user_id: uuid,
        lead,
        coordinator,
        pay,
        project: project.unwrap(),
        credits: credit.unwrap(),
        mid_grade: mid_grade.unwrap(),
        final_grade: final_grade.unwrap(),
    };

    let _user_id = edit_enrollment::EditEnrollment::execute(edit_variables).await;

    //We make another request here immediately after editing it, maybe not ideal speed wise.
    //Consider changing later?
    let enrollment_data =  EnrollmentByIds::get(uid, semester_id).await?;
   
    dbg!("{}", &enrollment_data);

    let mut form = Template::new(ENROLLMENT_EDIT_FORM);
    form.fields = json!({
        "data": enrollment_data,
    });

    form.in_page(&req, "Edit Enrollment").await

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnrollmentForm { 
    #[serde(default)]
    pub lead: bool,

    #[serde(default)]
    pub coordinator: bool,

    #[serde(default)]
    pub pay: bool,

    #[serde(default)]
    pub mid_grade: Option<f64>,

    #[serde(default)]
    pub final_grade: Option<f64>,

    #[serde(default)]
    pub credit: Option<i64>,

    pub project: Option<i64>, 
}
