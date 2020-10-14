use crate::templates::jumbotron::Jumbotron;
use crate::web::RequestContext;
use actix_web::HttpResponse;

/// 404 Page
pub async fn not_found(ctx: RequestContext) -> HttpResponse {
    HttpResponse::NotFound().body(
        Jumbotron::jumbotron_page(
            &ctx,
            "RCOS - 404",
            "404",
            "The page you're looking for does not seem to exist.",
        )
        .await,
    )
}
