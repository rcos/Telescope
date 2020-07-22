use crate::web::Template;

#[derive(Serialize)]
pub struct DeveloperssPage;

impl Template for DeveloperssPage {
    const TEMPLATE_NAME: &'static str = "static/developers";
}
