use crate::web::RequestContext;
use actix_web::web::Path;
use uuid::Uuid;
use actix_web::HttpResponse;

/// The service to display a user profile. The user is specified by the id in the
/// request path.
#[get("/profile/{uid}")]
pub async fn profile_service(ctx: RequestContext, user_id: Path<Uuid>) -> HttpResponse {
    unimplemented!()
}