use actix_web::{HttpRequest, HttpResponse};
use actix_session::Session;
use actix_web::web::Data;
use crate::web::app_data::AppData;

/// The service for the login page.
/// Receives only GET requests.
pub async fn login_service(
    req: HttpRequest,
    app_data: Data<AppData>,
    session: Session
) -> HttpResponse {
    unimplemented!()
}
