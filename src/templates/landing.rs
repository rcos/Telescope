use crate::web::Template;

#[derive(Serialize)]
pub struct LandingPage;

impl Template for LandingPage {
    const TEMPLATE_NAME: &'static str = "landing";
}
