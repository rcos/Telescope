//! Project page.
use crate::api::rcos::projects::projects_page;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use actix_web::HttpRequest;

const TEMPLATE_PATH: &'static str = "projects/list";

#[get("/projects")]
pub async fn get(req: HttpRequest) -> Result<Page, TelescopeError> {
    let projects = projects_page::AllProjects::get(0, None).await?;
    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({ "projects": projects.projects });
    return template.in_page(&req, "RCOS Projects").await;
}
