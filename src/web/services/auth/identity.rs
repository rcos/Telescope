//! Trait for types stored in the user's identity cookie.

use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::web::services::auth::oauth2_providers::{
    github::GitHubIdentity,
    discord::DiscordIdentity
};
use chrono::Utc;
use crate::error::TelescopeError;
use oauth2::RedirectUrl;
use crate::web::api::rcos::users::UserAccountType;

/// The top level enum stored in the identity cookie.
#[derive(Serialize, Deserialize)]
pub enum IdentityCookie {
    /// A GitHub access token.
    Github(GitHubIdentity),

    /// A Discord access and refresh token.
    Discord(DiscordIdentity),
}


impl IdentityCookie {
    /// If necessary, refresh an identity cookie. This could include getting a
    /// new access token from an OAuth API for example.
    pub fn refresh(self, redirect_uri: &RedirectUrl) -> Result<Self, TelescopeError> {
        // Destructure on discord identity.
        if let IdentityCookie::Discord(discord_identity) = self {
            return discord_identity.refresh(redirect_uri)
                .map(IdentityCookie::Discord);
        }

        // Otherwise return self -- Github does not need to be refreshed.
        return Ok(self);
    }

    /// Get the central RCOS API value representing this identity provider.
    pub fn user_account_type(&self) -> UserAccountType {
        match self {
            IdentityCookie::Discord(_) => UserAccountType::Discord,
            IdentityCookie::Github(_) => UserAccountType::GitHub,
        }
    }

    /// Get the platform's identity of the user who logged in to produce
    /// this access token.
    pub async fn get_account_identity(&self) -> Result<String, TelescopeError> {
        match self {
            IdentityCookie::Github(i) => i.get_user_id().await,
            IdentityCookie::Discord(i) => unimplemented!()
        }
    }
}
