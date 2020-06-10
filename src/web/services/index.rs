use crate::web::app_data::AppData;
use actix_session::Session;
use actix_web::{web as aweb, web::Data, HttpRequest, HttpResponse};
use crate::templates::page::{Page, Theme};
use crate::templates::navbar::Navbar;

/// Index / landing page.
/// All requests here will be GET.
pub async fn index_service(
    req: HttpRequest,
    session: Session,
    app_data: Data<AppData>,
) -> HttpResponse {
    let handlebars = &app_data.template_registry;
    let page_content = Navbar::new()
        .add_right_builder("/login", "Login", false);
    let page = Page::new(
        "RCOS",
        page_content.render(handlebars).unwrap(),
        Theme::Light
    );
    HttpResponse::Ok().body(page.render(handlebars).unwrap())
}
