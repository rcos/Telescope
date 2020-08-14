use crate::templates::page::Page;
use crate::web::{RequestContext, Template};
use actix_web::HttpResponse;
use serde::Serialize;

pub mod index;
pub mod sponsors;

/// A piece of static content that can be rendered in a Page object.
pub trait StaticPage: Serialize + Sized + Default {
    /// The path to the handlebars file.
    const TEMPLATE_NAME: &'static str;

    /// The title put at the top of the page.
    const PAGE_TITLE: &'static str;

    /// Render the default struct into a page, unwrapping all errors.
    fn render(ctx: &RequestContext) -> String {
        let s: Self = Self::default();
        let hbs = ctx.handlebars();
        let inner = hbs.render(Self::TEMPLATE_NAME, &s).unwrap();
        Page::new(Self::PAGE_TITLE, inner, ctx).render(hbs).unwrap()
    }

    /// Actix handler that can be used to generate responses.
    fn handle(ctx: RequestContext) -> HttpResponse {
        HttpResponse::Ok().body(Self::render(&ctx))
    }
}
