//! Calendar template. This is mostly a static template.

use crate::templates::{
    Template,
    page
};
use crate::error::TelescopeError;
use actix_web::HttpRequest;

/// The path to the template's handlebars file.
const TEMPLATE_NAME: &'static str = "calendar";

/// The page title of the calendar page.
const PAGE_TITLE: &'static str = "RCOS Calendar";

/// Create a calendar template in a page with the full calendar headers
/// included properly.
pub async fn calendar_page(req: &HttpRequest) -> Result<Template, TelescopeError> {
    // Make the calendar.
    let calendar = Template::new(TEMPLATE_NAME);
    // Put it in a page.
    return Ok(calendar.render_into_page(req, PAGE_TITLE)
        // Wait for the content of the page to render.
        .await?
        // Add the headers to the page.
        .field(page::INCLUDE_FULLCALENDAR, true));
}
