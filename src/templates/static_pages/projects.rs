use super::StaticPage;

/// Zero Sized Type linked to the static sponsors page content.
#[derive(Serialize, Default, Debug, Copy, Clone)]
pub struct ProjectsPage;

impl StaticPage for ProjectsPage {
    const TEMPLATE_NAME: &'static str = "static/projects";
    const PAGE_TITLE: &'static str = "RCOS Projects";
}
