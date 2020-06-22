use crate::templates::page::Page;
use crate::templates::static_pages::sponsors::SponsorsPage;
use crate::web::PageContext;
use actix_web::HttpResponse;

/// Sponsor page service. As far as I know, this doesn't change frequently.
/// As such this is static (until changed otherwise).
pub fn sponsors_page(pc: PageContext) -> HttpResponse {
    let ctx = &pc;
    let page = Page::new("RCOS Sponsors", pc.render(&SponsorsPage).unwrap(), ctx);
    HttpResponse::Ok().body(ctx.render(&page).unwrap())
}
