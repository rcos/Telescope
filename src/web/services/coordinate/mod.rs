//mod enrollments;
//mod project_pitches;
//mod meetings;

use crate::api::rcos::users::role_lookup::RoleLookup;
use crate::api::rcos::users::UserRole;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use crate::api::rcos::semesters::current::info::CurrentSemesters;
use crate::web::middlewares::authorization::{Authorization, AuthorizationResult};
use actix_web::guard;
use actix_web::web as aweb;
use actix_web::web::ServiceConfig;
use actix_web::HttpRequest;
use futures::future::LocalBoxFuture;
use uuid::Uuid;

fn coordinator_authorization(user_id: Uuid) -> LocalBoxFuture<'static, AuthorizationResult>{
    Box::pin(async move{
        //check that user is a coordinator+
        let role: UserRole = RoleLookup::get(user_id)
            .await?
            .expect("Viewer's account does not exist.");

        //forbid access if not the case
        if !role.is_coordinator() {
            Err(TelescopeError::Forbidden)
        } else {
            Ok(())
        }
    })
}

pub fn register(config: &mut ServiceConfig) {
    //Create coordinator auth middleware
    let coordinator_authorization_middleware: Authorization = Authorization::new(coordinator_authorization);
    
    //Coordinator Panel index page.
    config.service(
        aweb::resource("/coordinate")
        .guard(guard::Get())
        .wrap(coordinator_authorization_middleware.clone())
        .to(index),
        );

    config.service(
        aweb::scope("/coordinate/")
        .wrap(coordinator_authorization_middleware)
//        .configure(semesters::register),
        );
}

async fn index(req: HttpRequest) -> Result<Page, TelescopeError> {
    let current_semester_data = CurrentSemesters::get().await?; 

    let mut template = Template::new("coordinate/index");
    template.fields = json!({
        "data": current_semester_data,
    });
    template.in_page(&req, "Current Semesters").await
}
