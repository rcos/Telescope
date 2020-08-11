use crate::web::RequestContext;
use actix_web::HttpResponse;

/// Service to register a new user. Respond only to post requests.
pub async fn registration_service(ctx: RequestContext) -> HttpResponse {
    unimplemented!()
}