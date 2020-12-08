use crate::{
    templates::{
        jumbotron,
        Template
    },
    web::RequestContext
};

use actix_web::HttpResponse;

/// 404 Page.
pub async fn not_found(ctx: RequestContext) -> HttpResponse {
    let jumbo: Template =
        jumbotron::new("404", "The page you're looking for does not seem to exist.");

    HttpResponse::NotFound().body(ctx.render_in_page(&jumbo, "RCOS - 404").await)
}
