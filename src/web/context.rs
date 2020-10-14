use crate::{
    models::User,
    web::{
        api::graphql::ApiContext,
        app_data::AppData
    },
};

use actix_web::{
    dev::{Payload, PayloadStream},
    web::{block, Data},
    Error, FromRequest, HttpRequest,
};

use futures::future::{ok, Ready};

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

    /// Check if a user is logged in. Calls the database to check user valididty.
    pub async fn logged_in(&self) -> bool {
        let id = self.identity.identity()
            .and_then(|s| Uuid::parse_str(&s).ok());
        if let Some(uid) = id {
            User::get_from_db_by_id(self.get_db_connection().await, uid)
                .await
                .is_some()
        } else {
            false
        }
    }

    /// Get associated Handlebars template registry for manual template rendering.
    pub fn handlebars(&self) -> &Handlebars<'static> {
        self.app_data.template_registry.as_ref()
    }

    /// Render a template using the handlebars templates in this context.
    pub fn render<T: Template>(&self, template: &T) -> String {
        template
            .render(self.app_data.template_registry.as_ref())
            .map_err(|e| {
                error!("Handlebars rendering error: {}", e);
                e
            })
            .unwrap_or("Handlebars rendering error".into())
    }

    /// Asynchronously get a database connection.
    pub async fn get_db_connection(&self) -> DbConnection {
        let db_conn_pool: Pool<ConnectionManager<PgConnection>> = self.clone_connection_pool();
        block(move || {
            db_conn_pool.get().map_err(|e| {
                error!("Could not get database connection: {}", e);
                e
            })
        })
        .await
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
    pub async fn get_api_context(&self) -> Option<ApiContext> {
        ApiContext::new(self.clone_connection_pool(), self).await
    }

    /// Asynchronously get the logged in user if there is one.
    pub async fn user_identity(&self) -> Option<User> {
        match self.user_id_identity() {
            Some(uid) => User::get_from_db_by_id(self.get_db_connection().await, uid).await,
            None => None,
        }
    }

    /// Get the user id of the logged in user. This should be preferred
    /// to `get_user_identity` when possible to avoid an extra database query.
    pub fn user_id_identity(&self) -> Option<Uuid> {
        self.identity
            .identity()
            .and_then(|s| Uuid::parse_str(s.as_str()).ok())
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
