//! Project page.
use crate::api::rcos::projects::projects_page;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use actix_web::HttpRequest;

use crate::web::services::auth::identity::Identity;
use crate::api::rcos::projects::authorization_for::{AuthorizationFor, UserProjectAuthorization};

const TEMPLATE_PATH: &'static str = "projects/list";

#[get("/projects")]
pub async fn get(
    req: HttpRequest,
    identity: Identity,
) -> Result<Page, TelescopeError> {


    let viewer: Option<_> = identity.get_user_id().await?;

    //get their authorization level
    let authorization = AuthorizationFor::get(viewer).await?;

    let projects = projects_page::AllProjects::get(0, None).await?;
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({ 
        "projects": projects.projects,
        "authorization": authorization,
    });

    return template.in_page(&req, "RCOS Projects").await;
}
