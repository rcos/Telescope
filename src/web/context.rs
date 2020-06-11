use crate::web::app_data::AppData;
use actix_session::Session;
use actix_web::dev::{Payload, PayloadStream};
use actix_web::web::Data;
use actix_web::Error;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use handlebars::{Handlebars, RenderError};
use serde::Serialize;

/// Trait for renderable templates.
pub trait Template: Serialize + Sized {
    const TEMPLATE_NAME: &'static str;

    fn render(&self, handlebars: &Handlebars) -> Result<String, RenderError> {
        handlebars.render(Self::TEMPLATE_NAME, self)
    }
}

/// The items making up a page context (the context in which a request has been made.)
pub struct PageContext {
    app_data: Data<AppData>,
    request: HttpRequest,
    session: Session,
}

impl PageContext {
    /// Construct a new page context from a request and site data.
    pub fn new(data: Data<AppData>, request: HttpRequest, session: Session) -> Self {
        Self {
            app_data: data,
            request,
            session,
        }
    }

    /// Get the HttpRequest that originated this page context.
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    /// Get the associated user session (cookies) that originated with this page context.
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Render a template using the handlebars templates in this context.
    pub fn render<T: Template>(&self, template: &T) -> Result<String, RenderError> {
        template.render(self.app_data.template_registry.as_ref())
    }
}

impl FromRequest for PageContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload<PayloadStream>) -> Self::Future {
        let app_data = Data::<AppData>::from_request(req, payload)
            .into_inner()
            .unwrap();
        let request = HttpRequest::from_request(req, payload)
            .into_inner()
            .unwrap();
        let session = Session::from_request(req, payload).into_inner().unwrap();
        ok(Self::new(app_data, request, session))
    }
}
