use crate::web::app_data::AppData;
use actix_session::Session;
use actix_web::{
    dev::{
        Payload,
        PayloadStream
    },
    web::Data,
    Error,
    FromRequest,
    HttpRequest
};
use futures::future::{ok, Ready};
use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use diesel::{
    r2d2::{
        ConnectionManager,
        PooledConnection,
        Pool
    },
    PgConnection
};

/// Trait for renderable templates.
pub trait Template: Serialize + Sized {
    const TEMPLATE_NAME: &'static str;

    fn render(&self, handlebars: &Handlebars) -> Result<String, RenderError> {
        handlebars.render(Self::TEMPLATE_NAME, self)
    }
}

/// The items making up a page context (the context in which a request has been made.)
pub struct RequestContext {
    app_data: Data<AppData>,
    request: HttpRequest,
    session: Session,
}

impl RequestContext {
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

    /// Get associated Handlebars template registry for manual template rendering.
    pub fn handlebars(&self) -> &Handlebars<'static> {
        self.app_data.template_registry.as_ref()
    }

    /// Render a template using the handlebars templates in this context.
    pub fn render<T: Template>(&self, template: &T) -> Result<String, RenderError> {
        template.render(self.app_data.template_registry.as_ref())
    }

    /// Get a database connection. This may block for up to the amount of time specified
    /// in the connection pool config in `main.rs` (currently 15 sec).
    ///
    /// ## Panics:
    /// - If a database connection is not available.
    pub fn get_db_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        let db_conn_pool: &Pool<ConnectionManager<PgConnection>> = &self.app_data.db_connection_pool;
        db_conn_pool
            .get()
            .map_err(|e| {
                error!("Could not get database connection: {}", e);
                e
            })
            .unwrap()
    }
}

impl FromRequest for RequestContext {
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
