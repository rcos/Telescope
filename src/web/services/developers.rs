use crate::web::PageContext;
use actix_web::HttpResponse;
use crate::templates::page::Page;

/// Sponsor page service. As far as I know, this doesn't change frequently.
/// As such this is static (until changed otherwise).
pub fn developers_page(pc: PageContext) -> HttpResponse {
    let ctx = &pc;
    let page = Page::new(
        "RCOS Developers",
        r#"<object src="static/pages/sponsors.html"></object>"#,
        ctx);
    HttpResponse::Ok().body(ctx.render(&page).unwrap())
}