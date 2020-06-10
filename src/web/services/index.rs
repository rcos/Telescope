use actix_session::Session;
use actix_web::{
    HttpResponse,
    HttpRequest,
    web as aweb,
    web::Data
};
use crate::web::app_data::AppData;

/// Index / landing page
pub async fn index_service(req: HttpRequest, session: Session, app_data: Data<AppData>) -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}