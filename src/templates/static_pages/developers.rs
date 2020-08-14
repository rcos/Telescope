use crate::web::Template;

#[derive(Serialize)]
pub struct DevelopersPage;

impl Template for DevelopersPage {
    const TEMPLATE_NAME: &'static str = "static/developers";
}
