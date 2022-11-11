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
use crate::api::rcos::users::navbar_auth::Authentication;
use uuid::Uuid;

mod enrollments;
//mod project_pitches;
//mod meetings;

fn coordinator_authorization(user_id: Uuid) -> LocalBoxFuture<'static, AuthorizationResult>{
    Box::pin(async move{
        // We use navbar_auth here to see if the user is currently coordinating or is an is an
        // admin. Could probably do to rename navbar_auth?
        let auth = Authentication::get(user_id).await?;
        if !(auth.is_coordinating() || auth.is_admin()){
            Err(TelescopeError::Forbidden)
        }
        else{
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
        .configure(enrollments::register)
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
