//! Trait for types stored in the user's identity cookie.

use crate::error::TelescopeError;
use crate::web::api::rcos::send_query;
use crate::web::api::rcos::users::accounts::reverse_lookup::ReverseLookup;
use crate::web::api::rcos::users::UserAccountType;
use crate::web::services::auth::oauth2_providers::{
    discord::DiscordIdentity, github::GitHubIdentity,
};
use actix_identity::Identity as ActixIdentity;
use actix_web::dev::{Payload, PayloadStream};
use actix_web::{FromRequest, HttpRequest};
use futures::future::{ready, Ready, LocalBoxFuture};
use serde::Serialize;
use crate::web::services::auth::IdentityProvider;
use std::future::Future;

/// Trait for all identities stored in the top-level identity cookie.
pub trait AuthenticatedIdentity {
    /// The type that provides these authenticated identities.
    type Provider: IdentityProvider;

    /// The type of user account associated with this identity cookie.
    const USER_ACCOUNT_TYPE: UserAccountType;

    /// The type representing an authenticated user on this platform.
    type AuthenticatedUser;

    /// The future type returned by the function to get this platform's authenticated user.
    type AuthenticatedUserFuture: Future<Output = Result<Self::AuthenticatedUser, TelescopeError>>;

    /// The future type for the method to asynchronously fetch the authenticated RCOS username.
    type RcosUsernameFuture: Future<Output = Result<Option<String>, TelescopeError>>;

    /// Get the user object that is authenticated on this platform with this cookie.
    fn get_authenticated_user(&self) -> Self::AuthenticatedUserFuture;

    /// Get the username of the RCOS user associated with the authenticated account
    /// if one exists.
    fn get_rcos_username(&self) -> Self::RcosUsernameFuture;
}

/// The top level object stored in the identity cookie.
#[derive(Serialize, Deserialize)]
pub struct AuthenticatedIdentities {
    /// A GitHub access token.
    github: Option<GitHubIdentity>,

    /// A Discord access and refresh token.
    discord: Option<DiscordIdentity>,
}

impl AuthenticatedIdentities {
    /// If necessary, refresh an identity cookie. This could include getting a
    /// new access token from an OAuth API for example.
    pub async fn refresh(mut self) -> Result<Self, TelescopeError> {
        // When there is a discord identity.
        if let Some(discord_identity) = self.discord {
            // Refresh the discord identity
            let refreshed = discord_identity.refresh().await?;
            // Store back and return self.
            self.discord = Some(refreshed);
            return Ok(self);
        }

        // Otherwise return self
        return Ok(self);
    }

    /// This identity is only valid if it is authenticated with at least one
    /// identity provider.
    pub fn is_valid(&self) -> bool {
        self.discord.is_some() || self.github.is_some()
    }

    /// Get the RCOS username of an authenticated user.
    pub async fn get_rcos_username(&self) -> Result<Option<String>, TelescopeError> {
        // Try first to get a username via the discord identity.
        if let Some(discord) = self.discord.as_ref() {
            let rcos_username: Option<String> = discord.get_rcos_username().await?;
            if rcos_username.is_some() {
                return Ok(rcos_username);
            }
        }

        // If there is no discord identity (or it's not linked) try with the
        // github identity.
        if let Some(github) = self.github.as_ref() {
            let rcos_username: Option<String> = github.get_rcos_username().await?;
            if rcos_username.is_some() {
                return Ok(rcos_username);
            }
        }

        // If neither worked out, return none.
        return Ok(None);
    }

    /// Try to get a specific type of authenticated identity from this cookie.
    pub fn get<T: AuthenticatedIdentity>(&self) -> Option<&T> {
        match T::USER_ACCOUNT_TYPE {
            UserAccountType::GitHub => self.github.as_ref(),
            UserAccountType::Discord => self.discord.as_ref(),
            _ => unimplemented!()
        }
    }
}

/// The identity of a user accessing telescope.
#[derive(Clone)]
pub struct Identity {
    /// The actix identity of this request. This handles cookie and
    /// security stuff.
    inner: ActixIdentity,
}

impl FromRequest for Identity {
    type Error = TelescopeError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
        // Extract the actix identity and convert any errors
        ready(
            ActixIdentity::extract(req)
                // Unwrap the ready future
                .into_inner()
                // Normalize the error as an ISE
                .map_err(|e| {
                    TelescopeError::ise(format!(
                        "Could not extract identity \
            object from request. Internal error: {}",
                        e
                    ))
                })
                // Wrap the extracted identity.
                .map(|inner| Self { inner }),
        )
    }
}

impl FromRequest for AuthenticatedIdentities {
    type Error = TelescopeError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
        // Clone a reference to the HTTP req, since its behind an Rc pointer.
        let owned_request: HttpRequest = req.clone();
        return Box::pin(async move {
            // Extract the telescope-identity from the request
            Identity::extract(&owned_request)
                // Wait and propagate errors
                .await?
                // Get the cookie if there is one
                .identity()
                // Wait and make error on none
                .await
                .ok_or(TelescopeError::NotAuthenticated)
        });
    }
}

impl Identity {
    /// Forget the user's identity if it exists.
    pub fn forget(&self) {
        self.inner.forget()
    }

    /// Save an identity object to the client's cookies.
    pub fn save(&self, identity: &AuthenticatedIdentities) {
        // Serialize the cookie to JSON first. This serialization should not fail.
        let cookie: String =
            serde_json::to_string(identity).expect("Could not serialize identity cookie");

        // Remember cookie.
        self.inner.remember(cookie)
    }

    /// Get the user's identity. Refresh it if necessary.
    pub async fn identity(&self) -> Option<AuthenticatedIdentities> {
        // Get the inner identity as a String.
        let id: String = self.inner.identity()?;
        // try to deserialize it
        match serde_json::from_str::<AuthenticatedIdentities>(id.as_str()) {
            // On okay, refresh the identity cookie if needed
            Ok(id) => match id.refresh().await {
                // If this succeeds
                Ok(id) => {
                    // Check that this identity us authenticated with at least
                    // one provider.
                    if id.is_valid() {
                        // If so, save and return it.
                        self.save(&id);
                        return Some(id);
                    } else {
                        // If not forget it.
                        self.forget();
                        return None;
                    }
                }

                // If it fails to refresh, we have no identity. Send a warning
                // and return None.
                Err(e) => {
                    warn!("Could not refresh identity token. Error: {}", e);
                    return None;
                }
            },

            // If there is an error deserializing, the identity is malformed.
            // Forget it, and log a warning. Return no identity.
            Err(err) => {
                warn!("Bad identity forgotten. Error: {}", err);
                self.forget();
                return None;
            }
        }
    }

    /// Get the username of the authenticated RCOS account (if there is one.)
    pub async fn get_rcos_username(&self) -> Result<Option<String>, TelescopeError> {
        // If there is an identity cookie
        if let Some(id) = self.identity().await {
            // Use it to get the authenticated RCOS username.
            return id.get_rcos_username().await;
        } else {
            return Ok(None);
        }
    }
}

