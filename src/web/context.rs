use crate::{
    web::{
        api::ApiContext,
        app_data::AppData
    },
    models::User
};

use actix_web::{
    dev::{Payload, PayloadStream},
    web::{
        Data,
        block
    },
    Error,
    FromRequest,
    HttpRequest,
};

use futures::future::{
    ok,
    err,
    Ready,
    FutureExt,
    TryFutureExt,
    BoxFuture
};

use handlebars::{Handlebars, RenderError};

use serde::Serialize;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

use actix_identity::Identity;
use uuid::Uuid;

/// Trait for renderable templates.
pub trait Template: Serialize + Sized {
    const TEMPLATE_NAME: &'static str;

    fn render(&self, handlebars: &Handlebars) -> Result<String, RenderError> {
        handlebars.render(Self::TEMPLATE_NAME, self)
    }
}

/// Database connection type.
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// The items making up a page context (the context in which a request has been made.)
pub struct RequestContext {
    app_data: Data<AppData>,
    request: HttpRequest,
    identity: Identity,
}

impl RequestContext {
    /// Construct a new page context from a request and site data.
    pub fn new(data: Data<AppData>, request: HttpRequest, identity: Identity) -> Self {
        Self {
            app_data: data,
            request,
            identity,
        }
    }

    /// Get the HttpRequest that originated this page context.
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    /// Get the associated user session (cookies) that originated with this page context.
    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    /// Check if a user is logged in (via identity)
    pub fn logged_in(&self) -> bool {self.identity.identity().is_some()}

    /// Get associated Handlebars template registry for manual template rendering.
    pub fn handlebars(&self) -> &Handlebars<'static> {
        self.app_data.template_registry.as_ref()
    }

    /// Render a template using the handlebars templates in this context.
    pub fn render<T: Template>(&self, template: &T) -> String {
        template.render(self.app_data.template_registry.as_ref())
            .map_err(|e| {
                error!("Handlebars rendering error: {}", e);
                e
            })
            .unwrap_or("Handlebars rendering error".into())
    }

    /// Get a database connection. This may block for up to the amount of time specified
    /// in the connection pool config in `main.rs` (currently 15 sec).
    /// If a connection is not available, this method will panic.
    pub fn get_db_connection(&self) -> DbConnection {
        let db_conn_pool: &Pool<ConnectionManager<PgConnection>> =
            &self.app_data.db_connection_pool;
        db_conn_pool
            .get()
            .map_err(|e| {
                error!("Could not get database connection: {}", e);
                e
            })
            .unwrap()
    }

    /// Clone the connection pool. This can be useful for async operation,
    /// as this Context is not threadsafe (since HTTPRequest isn't) but Connection
    /// pools are.
    pub fn clone_connection_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.app_data.db_connection_pool.clone()
    }

    /// Get an API context object (a partial sub-context of this context) to execute
    /// GraphQL API requests in.
    pub fn get_api_context(&self) -> Option<ApiContext> {
        ApiContext::new(self.clone_connection_pool(), self)
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
        let identity = Identity::from_request(req, payload).into_inner().unwrap();
        ok(Self::new(app_data, request, identity))
    }
}

/// The user currently logged in.
impl FromRequest for User {
    // We loose actix's error reporting here because actix-web's errors are not thread
    // safe for some reason.
    type Error = ();
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload<PayloadStream>) -> Self::Future {
        let ctx = RequestContext::from_request(req, payload)
            .into_inner()
            // this should unwrap fine as long as the implementation above
            // always returns ok
            .unwrap();

        let db_conn_pool = ctx.clone_connection_pool();
        let uid_res = ctx
            .identity
            .identity()
            .ok_or(())
            .and_then(|s| Uuid::parse_str(s.as_str()).map_err(|_| ()));

        if uid_res.is_err() {
            return err(uid_res.err().unwrap()).boxed();
        }

        let uid = uid_res.unwrap();

        block(move || db_conn_pool.get())
            .boxed()
            .map_err(move |_| ())
            .boxed()
            .and_then(move |conn: DbConnection| {
                User::get_from_db_by_id(conn, uid)
                    .then(|opt| async { opt.ok_or(()) })
            })
            .boxed()
    }
}