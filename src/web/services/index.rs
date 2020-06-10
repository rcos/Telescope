use crate::web::app_data::AppData;
use actix_session::Session;
use actix_web::{web as aweb, web::Data, HttpRequest, HttpResponse};

/// Index / landing page
pub async fn index_service(
    req: HttpRequest,
    session: Session,
    app_data: Data<AppData>,
) -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}
