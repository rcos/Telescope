use actix_identity::Identity;
use actix_web::{
    dev::{Payload, PayloadStream},
    Error,
    FromRequest, HttpRequest, web::{block, Data},
};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use futures::future::{ready, Ready};
use handlebars::Handlebars;
use lettre::SendableEmail;
use lettre_email::Mailbox;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    models::users::User,
    templates::{page, Template},
    web::{api::graphql::ApiContext},
};
use crate::app_data::AppData;

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
        let id = self
            .identity
            .identity()
            .and_then(|s| Uuid::parse_str(&s).ok());
        if let Some(uid) = id {
            let db_res: Option<User> = User::get_from_db_by_id(self.get_db_conn().await, uid).await;

            if db_res.is_some() {
                true
            } else {
                // bad uuid in identity
                self.identity.forget();
                false
            }
        } else {
            false
        }
    }

    /// Get associated Handlebars template registry for manual template rendering.
    pub fn handlebars(&self) -> &Handlebars<'static> {
        self.app_data.template_registry.as_ref()
    }

    /// Render a template using the handlebars templates in this context.
    pub fn render(&self, template: &Template) -> String {
        template.render(self.app_data.template_registry.as_ref())
    }

    /// Asynchronously get a database connection.
    pub async fn get_db_conn(&self) -> DbConnection {
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
        self.app_data.clone_db_conn_pool()
    }

    /// Get an API context object (a partial sub-context of this context) to execute
    /// GraphQL API requests in.
    pub async fn get_api_context(&self) -> Option<ApiContext> {
        ApiContext::new(self.clone_connection_pool(), self).await
    }

    /// Asynchronously get the logged in user if there is one.
    pub async fn user_identity(&self) -> Option<User> {
        match self.user_id_identity() {
            Some(uid) => User::get_from_db_by_id(self.get_db_conn().await, uid).await,
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

    /// Render a page with the specified template as the page content and the title as specified.
    pub async fn render_in_page(
        &self,
        template: &Template,
        page_title: impl Into<Value>,
    ) -> String {
        let page: Template = page::of(&self, page_title, template).await;
        self.render(&page)
    }

    /// Send an email using the internal app data mailers derived from the
    /// server config.
    pub async fn send_mail<M>(&self, mail: M) -> Result<(), ()>
    where
        M: Into<SendableEmail> + Clone + Send + Sync + 'static,
    {
        self.app_data.send_mail(mail).await
    }

    /// Get the mail sender from the app data config.
    pub fn email_sender(&self) -> Mailbox {
        self.app_data.mail_sender.clone()
    }

    /// Extract the components of a context object and build it from
    /// an http request. This exists for the request extractor trait,
    /// which doesn't allow for the `?` operator.
    fn extract(req: &HttpRequest, payload: &mut Payload<PayloadStream>) -> Result<Self, Error> {
        let app_data: Data<AppData> = Data::<AppData>::from_request(req, payload).into_inner()?;
        let request: HttpRequest = HttpRequest::from_request(req, payload).into_inner()?;
        let identity: Identity = Identity::from_request(req, payload).into_inner()?;
        Ok(Self::new(app_data, request, identity))
    }
}

impl FromRequest for RequestContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload<PayloadStream>) -> Self::Future {
        ready(RequestContext::extract(req, payload))
    }
}
