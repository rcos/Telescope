//! Meetings page template. This is mostly a static template.

use crate::templates::{
    Template,
    page
};
use crate::error::TelescopeError;
use actix_web::HttpRequest;

/// The path to the template's handlebars file.
const TEMPLATE_NAME: &'static str = "meetings_page";

