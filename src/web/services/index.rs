use crate::web::app_data::AppData;
use actix_session::Session;
use actix_web::{web as aweb, web::Data, HttpRequest, HttpResponse};
use crate::templates::{
    page::Page,
    navbar::Navbar,
};

/// Index / landing page.
/// All requests here will be GET.
pub async fn index_service(
    req: HttpRequest,
    session: Session,
    app_data: Data<AppData>,
) -> HttpResponse {
    let handlebars = &app_data.template_registry;
    let page_content = Navbar::new()
        .add_right_builder("/login", "Login", "");
    let page = Page::new(
        "RCOS",
        page_content.render(handlebars).unwrap(),
    );
    HttpResponse::Ok().body(page.render(handlebars).unwrap())
}
