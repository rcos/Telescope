use crate::error::TelescopeError;
use crate::templates::Template;
use actix_web::HttpRequest;
use futures::future::LocalBoxFuture;
use crate::templates::page::Page;

pub mod sponsors;

/// A piece of static content. This is just a reference to a
/// handlebars file and some metadata for rendering the page.
pub trait StaticPage {
    /// The path to the handlebars file.
    const TEMPLATE_NAME: &'static str;

    /// The title of this page.
    const PAGE_TITLE: &'static str;

    /// Make the static template that this refers to.
    fn template() -> Template {
        Template::new(Self::TEMPLATE_NAME)
    }

    /// Create a page containing the static content. This is also the actix handler
    fn page(req: HttpRequest) -> LocalBoxFuture<'static, Result<Page, TelescopeError>> {
        Box::pin(async move {
            // We have to double wrap this future to avoid lifetime constraint issue?
            // Or at least adding the async block seems to fix it since it moves the template.
            Page::new(&req, Self::PAGE_TITLE, Self::template()).await
        })
    }
}
