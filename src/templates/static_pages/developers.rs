use super::StaticPage;

/// Zero Sized Type linked to the static sponsors page content.
#[derive(Serialize, Default, Debug, Copy, Clone)]
pub struct DevelopersPage;

impl StaticPage for DevelopersPage {
    const TEMPLATE_NAME: &'static str = "static/developers";
    const PAGE_TITLE: &'static str = "RCOS Developers";
}
