use crate::web::PageContext;
use actix_web::HttpResponse;

/// The service for the login page.
/// Receives only GET requests.
pub async fn login_service(pc: PageContext) -> HttpResponse {
    unimplemented!()
}
