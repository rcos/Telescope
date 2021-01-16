//! Internal server error response page.

use crate::{
    templates::{jumbotron, Template},
    web::RequestContext,
};
use actix_web::HttpResponse;

const TITLE: &'static str = "Error";
const HEADING: &'static str = "500 - Internal Server Error";
const MESSAGE: &'static str = "We encountered an error while processing your \
request. Please try again If you continue to have issues, please make a github \
issue and/or contact a server administrator.";

/// Create the page template to represent the ISE page.
pub async fn ise_page(ctx: &RequestContext) -> Template {
    jumbotron::page(ctx, TITLE, HEADING, MESSAGE).await
}

/// Create and render the ISE page.
async fn ise_page_str(ctx: &RequestContext) -> String {
    ctx.render(&ise_page(ctx).await)
}

/// Generate the Internal Server Error Http Response and Page.
pub async fn ise(ctx: &RequestContext) -> HttpResponse {
    HttpResponse::InternalServerError().body(ise_page_str(ctx).await)
}
