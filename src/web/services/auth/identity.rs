//! Trait for types stored in the user's identity cookie.

use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::web::services::auth::oauth2_providers::{
    github::GitHubIdentity,
    discord::DiscordIdentity
};

/// The top level enum stored in the identity cookie.
#[derive(Serialize, Deserialize)]
pub enum IdentityCookie {
    /// A GitHub access token.
    Github(GitHubIdentity),

    /// A Discord access and refresh token.
    Discord(DiscordIdentity),
}
