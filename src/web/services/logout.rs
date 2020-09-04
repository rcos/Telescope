use crate::web::RequestContext;
use actix_web::HttpResponse;
use crate::templates::static_pages::index::LandingPage;
use crate::templates::StaticPage;

/// Log a user out.
pub async fn logout_service(req_ctx: RequestContext) -> HttpResponse {
    let identity = req_ctx.identity();
    identity.forget();
    HttpResponse::Ok().body(LandingPage::render(&req_ctx))
}