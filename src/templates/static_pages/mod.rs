use crate::{
    templates::{page, Template},
    RequestContext,
};
use actix_web::HttpResponse;
use std::marker::PhantomData;

pub mod index;
pub mod ise;
pub mod projects;
pub mod sponsors;

/// An intermediate workaround structure to deal with the lack of support
/// for async functions in traits. This is a zero-sized structure that holds
/// a phantom for the type parameter.
pub struct Static<T: StaticPage> {
    phantom: PhantomData<T>,
}

impl<T: StaticPage> Static<T> {
    fn template() -> Template {
        Template::new(T::TEMPLATE_NAME)
    }
    /// Create a page containing the static content.
    async fn page(ctx: &RequestContext) -> Template {
        page::of(ctx, T::PAGE_TITLE, &Self::template()).await
    }

    /// Actix handler that can be used to generate responses.
    pub async fn handle(ctx: RequestContext) -> HttpResponse {
        let body = Self::page(&ctx).await;
        HttpResponse::Ok().body(ctx.render(&body))
    }
}

/// A piece of static content. This currently is just a reference to a
/// handlebars file and some metadata for rendering the page.
pub trait StaticPage {
    /// The path to the handlebars file.
    const TEMPLATE_NAME: &'static str;

    /// The title of this page.
    const PAGE_TITLE: &'static str;
}
