use crate::web::Template;

#[derive(Serialize)]
pub struct ProjectsPage;

impl Template for ProjectsPage {
    const TEMPLATE_NAME: &'static str = "static/projects";
}
