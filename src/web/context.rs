use actix_identity::Identity;
use actix_web::{
    dev::{Payload, PayloadStream},
    Error,
    FromRequest, HttpRequest, web::{block, Data},
};
use futures::future::{ready, Ready};
use handlebars::Handlebars;
use lettre::SendableEmail;
use lettre_email::Mailbox;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    templates::{page, Template},
};
use crate::app_data::AppData;
use crate::error::TelescopeError;


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

    /// Get the user id of the logged in user. This does not check if the
    /// user exists in the database though, so avoid
    /// using it in favor of [`user_identity`] where possible.
    fn identity_user_id(&self) -> Option<Uuid> {
        self.identity
            .identity()
            .and_then(|s| Uuid::parse_str(s.as_str()).ok())
            .or_else(|| {
                // If there is no identity or the identity is malformed,
                // forget it.
                self.identity.forget();
                None
            })
    }

    /// Asynchronously get the logged in user if there is one.
    pub async fn user_identity(&self) -> Result<Option<User>, TelescopeError> {
        match self.identity_user_id() {
            Some(uid) => User::get_from_db_by_id(uid).await,
            None => Ok(None),
        }
    }

    /// Check if a user is logged in. Calls the database to check user valididty.
    pub async fn logged_in(&self) -> Result<bool, TelescopeError> {
        self.user_identity()
            .await
            .map(Option::is_some)
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
