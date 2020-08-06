use crate::templates::page::Page;
use crate::templates::static_pages::developers::DevelopersPage;
use crate::web::RequestContext;
use actix_web::HttpResponse;

/// Developer page service.
pub fn developers_page(pc: RequestContext) -> HttpResponse {
    let ctx = &pc;
    let page = Page::new("RCOS Developers", pc.render(&DevelopersPage).unwrap(), ctx);
    HttpResponse::Ok().body(ctx.render(&page).unwrap())
}
