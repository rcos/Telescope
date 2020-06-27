use crate::templates::page::Page;
use crate::web::RequestContext;
use actix_web::HttpResponse;

/// Index / landing page.
/// All requests here will be GET.
pub async fn index_service(pc: RequestContext) -> HttpResponse {
    let page = Page::new("RCOS", "Hello World", &pc);
    HttpResponse::Ok().body(pc.render(&page).unwrap())
}
