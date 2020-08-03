use super::StaticPage;

/// Landing Page.
#[derive(Serialize, Copy, Clone, Debug, Default)]
pub struct LandingPage;

impl StaticPage for LandingPage {
    const TEMPLATE_NAME: &'static str = "static/index";
    const PAGE_TITLE: &'static str = "RCOS";
}
