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
        } else {
            return Ok(self);
        }
    }
}