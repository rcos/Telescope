use crate::web::RequestContext;
use actix_web::HttpResponse;

/// Log a user out.
pub async fn logout_service(req_ctx: RequestContext) -> HttpResponse {
    let identity = req_ctx.identity();
    identity.forget();
    HttpResponse::Ok().finish()
}