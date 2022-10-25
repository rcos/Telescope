use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::api::rcos::users::enrollments::enrollment_by_ids::EnrollmentByIds;
use crate::templates::Template;
use actix_web::web::Form;
use crate::api::rcos::users::enrollments::edit_enrollment::edit_enrollment::Variables;
use crate::api::rcos::users::enrollments::edit_enrollment;
use actix_web::web::{Path, ServiceConfig, HttpRequest};
use crate::api::rcos::prelude::*;
use serde::{
    de,
    Deserialize
};

const ENROLLMENT_EDIT_FORM: &str = "coordinate/enrollments/edit/form";

//there is one potential issue with this page, a ooordinator probably shouldn't be able to edit
//their own enrollment or the enrollments of other coordinators, while there won't be an edit
//button on those restricted enrollments for coordinators, they can access the edit page for those
//enrollments if they put in the url directly.  I don't have a solution for this right now
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
    let uuid = user_id.parse::<uuid>().ok().unwrap();

    let EnrollmentForm {
        lead,
        coordinator,
        pay,
        project,
        credit,
        mid_grade,
        final_grade,
    } = form_data;

    let edit_variables = Variables {
        semester_id: semester_id.clone(),
        user_id: uuid,
        lead,
        coordinator,
        pay,
        project,
        credits: credit,
        mid_grade,
        final_grade,
    };

    let _user_id = edit_enrollment::EditEnrollment::execute(edit_variables).await;

    //We make another request here immediately after editing it, maybe not ideal speed wise.
    //Consider changing later?
    let enrollment_data =  EnrollmentByIds::get(uuid, semester_id).await?;
   
    let mut form = Template::new(ENROLLMENT_EDIT_FORM);
    form.fields = json!({
        "data": enrollment_data,
    });

    form.in_page(&req, "Edit Enrollment").await

}

// Modification of a snippet found on an Actix Web github issue. Helps with null value numbers
pub fn deserialize_option_ignore_error<'de, T, D>(d: D) -> Result<Option<T>, D::Error>
where
    T: de::Deserialize<'de>,
    D: de::Deserializer<'de>,
{
    let res = T::deserialize(d);
    match res{
        Ok(..) => Ok(res.ok()),
        Err(..) => Ok(None),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnrollmentForm { 
    #[serde(default)]
    pub lead: bool,

    #[serde(default)]
    pub coordinator: bool,

    #[serde(default)]
    pub pay: bool,

    #[serde(default, deserialize_with = "deserialize_option_ignore_error")]
    pub mid_grade: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_ignore_error")]
    pub final_grade: Option<f64>,

    #[serde(default, deserialize_with = "deserialize_option_ignore_error")]
    pub credit: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_option_ignore_error")]
    pub project: Option<i64>, 
}
