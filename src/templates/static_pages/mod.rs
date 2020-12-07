use crate::{
    templates::page::Page,
    RequestContext
};
use actix_web::HttpResponse;
use serde::Serialize;

pub mod developers;
pub mod index;
pub mod projects;
pub mod sponsors;

/// An intermediate workaround structure to deal with the lack of support
/// for async functions in traits.
pub struct Static<T: StaticPage> {
    page_content: T,
}

impl<T: StaticPage> Static<T> {
    /// Create a page containing the static content.
    async fn in_page(&self, ctx: &RequestContext) -> Page {
        Page::new(T::PAGE_TITLE, &self.page_content, ctx).await
    }

    /// Actix handler that can be used to generate responses.
    pub async fn handle(ctx: RequestContext) -> HttpResponse {
        let body = T::normalized_default()
            .in_page(&ctx)
            .await
            .into();
        HttpResponse::Ok().body(ctx.render(&body))
    }
}

/// A piece of static content that can be rendered in a Page object.
pub trait StaticPage: Serialize + Sized + Default {
    /// The path to the handlebars file.
    const TEMPLATE_NAME: &'static str;

    /// The title put at the top of the page.
    const PAGE_TITLE: &'static str;

    fn normalized(self) -> Static<Self> {
        Static { page_content: self }
    }

    fn normalized_default() -> Static<Self> {
        Self::default().normalized()
    }
}
