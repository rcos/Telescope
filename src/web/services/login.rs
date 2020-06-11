use actix_web::HttpResponse;
use crate::web::PageContext;

/// The service for the login page.
/// Receives only GET requests.
pub async fn login_service(pc: PageContext) -> HttpResponse {
    unimplemented!()
}
