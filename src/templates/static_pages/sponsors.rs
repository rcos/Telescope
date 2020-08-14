use super::StaticPage;

/// Zero Sized Type linked to the static sponsors page content.
#[derive(Serialize, Default, Debug, Copy, Clone)]
pub struct SponsorsPage;

impl StaticPage for SponsorsPage {
    const TEMPLATE_NAME: &'static str = "static/sponsors";
    const PAGE_TITLE: &'static str = "RCOS Sponsors";
}
