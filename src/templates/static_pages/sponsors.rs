use crate::web::Template;

#[derive(Serialize)]
pub struct SponsorsPage;

impl Template for SponsorsPage {
    const TEMPLATE_NAME: &'static str = "static/sponsors";
}