use crate::templates::page::Page;
use crate::templates::static_pages::projects::ProjectsPage;
use crate::web::RequestContext;
use actix_web::HttpResponse;

/// Project page service.
pub fn projects_page(pc: RequestContext) -> HttpResponse {
    let ctx = &pc;
    let page = Page::new("RCOS Projects", pc.render(&ProjectsPage).unwrap(), ctx);
    HttpResponse::Ok().body(ctx.render(&page).unwrap())
}
