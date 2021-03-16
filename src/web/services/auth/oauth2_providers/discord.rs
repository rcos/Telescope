//! Discord OAuth2 flow.

use crate::env::global_config;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::{AuthenticatedIdentities, AuthenticatedIdentity};
use crate::web::services::auth::oauth2_providers::Oauth2IdentityProvider;
use crate::web::services::auth::IdentityProvider;
use actix_web::http::header::ACCEPT;
use chrono::{DateTime, Duration, Utc};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AccessToken, RefreshToken, Scope, TokenResponse};
use oauth2::{AuthUrl, TokenUrl};
use serenity::model::user::CurrentUser;
use std::sync::Arc;
use crate::web::api::rcos::users::accounts::reverse_lookup::ReverseLookup;
use crate::web::api::rcos::users::UserAccountType;
use crate::web::api::rcos::send_query;
use std::future::Future;
use futures::future::LocalBoxFuture;

/// The Discord API endpoint to query for user data.
const DISCORD_API_ENDPOINT: &'static str = "https://discord.com/api/v8";

/// Zero-sized type used to represent Discord based identity verification.
pub struct DiscordOAuth;

/// The object stored in a user's cookies when authenticated via discord.
#[derive(Serialize, Deserialize)]
pub struct DiscordIdentity {
    /// The OAuth2 access token granted by discord.
    access_token: AccessToken,
    /// When the access token expires.
    expiration: DateTime<Utc>,
    /// The token to use to refresh it.
    refresh_token: RefreshToken,
}

lazy_static! {
    static ref DISCORD_CLIENT: Arc<BasicClient> = {
        // Get the global config.
        let config = global_config();

        // Create GitHub OAuth2 client.
        let client = BasicClient::new(
            config.discord_config.client_id.clone(),
            Some(config.discord_config.client_secret.clone()),
            AuthUrl::new("https://discord.com/api/oauth2/authorize".into())
                .expect("Invalid Discord Auth URL"),
            Some(TokenUrl::new("https://discord.com/api/oauth2/token".into())
                .expect("Invalid Discord Token URL")));

        // Return the client config wrapped in an Arc.
        Arc::new(client)
    };
}

impl Oauth2IdentityProvider for DiscordOAuth {
    const SERVICE_NAME: &'static str = "discord";
    const USER_ACCOUNT_TYPE: UserAccountType = DiscordIdentity::USER_ACCOUNT_TYPE;

    fn get_client() -> Arc<BasicClient> {
        DISCORD_CLIENT.clone()
    }

    fn scopes() -> Vec<Scope> {
        vec![
            // Scope required for us to get the users identity.
            Scope::new("identify".to_owned()),
        ]
    }

    fn make_identity(token_response: &BasicTokenResponse) -> AuthenticatedIdentities {
        AuthenticatedIdentities::Discord(DiscordIdentity::from_response(token_response))
    }
}

impl DiscordIdentity {
    fn from_response(token_response: &BasicTokenResponse) -> Self {
        // Unwrap the token duration.
        let token_duration = token_response
            .expires_in()
            .expect("Discord did not return token duration.");
        // Convert the token duration to a chrono duration.
        let chrono_duration =
            Duration::from_std(token_duration).expect("Token duration out of range.");

        DiscordIdentity {
            access_token: token_response.access_token().clone(),
            expiration: Utc::now() + chrono_duration,
            refresh_token: token_response
                .refresh_token()
                .expect("Discord did not return refresh token.")
                .clone(),
        }
    }

    /// Refresh this access token if necessary.
    pub async fn refresh(self) -> Result<Self, TelescopeError> {
        // If this token has expired
        if self.expiration < Utc::now() {
            // Get a discord client and make a refresh token request.
            let client: Arc<BasicClient> = <DiscordOAuth as Oauth2IdentityProvider>::get_client();
            let mut refresh_token_request = client.exchange_refresh_token(&self.refresh_token);
            // Add scopes.
            for scope in DiscordOAuth::scopes() {
                refresh_token_request = refresh_token_request.add_scope(scope);
            }
            // Create refresh response
            let response = refresh_token_request
                // Add login redirect path.
                .add_extra_param("redirect_uri", DiscordOAuth::login_redirect_path().as_str())
                // Send the request.
                .request(oauth2::reqwest::http_client)
                // Handle and propagate the error.
                .map_err(|err| {
                    TelescopeError::ise(format!(
                        "Could not refresh Discord OAuth2 token. Error: {}",
                        err
                    ))
                })?;

            // Make and return the new token.
            return Ok(Self::from_response(&response));
        } else {
            // We don't need to refresh -- return self.
            return Ok(self);
        }
    }

    /// Get the authenticated Discord account's ID.
    pub async fn get_user_id(&self) -> Result<String, TelescopeError> {
        self.get_authenticated_user().await.map(|u| u.id.to_string())
    }

    /// Get the RCOS username of the account associated with the authenticated
    /// discord user if one exists.
    pub async fn get_rcos_username(&self) -> Result<Option<String>, TelescopeError> {
        // Get the authenticated user id.
        let platform_id: String = self.get_user_id().await?;
        // Build the query variables for a reverse lookup query to the central RCOS API
        let variables = ReverseLookup::make_vars(UserAccountType::Discord, platform_id);
        // Send the query and await the response
        return send_query::<ReverseLookup>(None, variables)
            .await
            .map(|response| response.username());
    }
}

impl AuthenticatedIdentity for DiscordIdentity {
    type Provider = DiscordOAuth;
    const USER_ACCOUNT_TYPE: UserAccountType = UserAccountType::Discord;
    type AuthenticatedUser = CurrentUser;
    type AuthenticatedUserFuture = LocalBoxFuture<'static, Result<Self::AuthenticatedUser, TelescopeError>>;
    type RcosUsernameFuture = LocalBoxFuture<'static, Result<Option<String>, TelescopeError>>;

    fn get_authenticated_user(&self) -> Self::AuthenticatedUserFuture {
        // Clone the access token to be owned by the future
        let secret: String = self.access_token.secret().clone();

        return Box::pin(async move {
            // Send the GET request to the discord API.
            return reqwest::Client::new()
                .get(format!("{}/users/@me", DISCORD_API_ENDPOINT).as_str())
                .bearer_auth(secret)
                .header(ACCEPT, "application/json")
                .send()
                .await
                .map_err(|e| {
                    TelescopeError::ise(format!(
                        "Could not send identification query to Discord \
                API. Internal error: {}",
                        e
                    ))
                })?
                .json::<CurrentUser>()
                .await
                .map_err(|e| {
                    TelescopeError::ise(format!(
                        "Error with identification response from Discord \
                API. Internal error: {}",
                        e
                    ))
                });
        });
    }

    fn get_rcos_username(&self) -> Self::RcosUsernameFuture {
        unimplemented!()
    }
}
