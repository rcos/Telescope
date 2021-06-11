use crate::api::rcos::users::accounts::reverse_lookup::ReverseLookup;
use crate::api::rcos::users::UserAccountType;
use crate::api::{
    github::{
        self,
        users::authenticated_user::{
            authenticated_user::{AuthenticatedUserViewer, Variables},
            AuthenticatedUser,
        },
    },
    rcos,
};
use crate::env::global_config;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::{RootIdentity, AuthenticationCookie};
use crate::web::services::auth::oauth2_providers::{Oauth2IdentityProvider, Oauth2Identity};
use futures::future::LocalBoxFuture;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AccessToken, AuthUrl, Scope, TokenResponse, TokenUrl};
use std::sync::Arc;

/// Zero sized type representing the GitHub OAuth2 identity provider.
pub struct GitHubOauth;

/// The identity object stored in the user's cookies for users signed in via
/// GitHub.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitHubIdentity {
    /// The OAuth2 Access token granted by GitHub.
    pub access_token: AccessToken,
}

// Lazy static github client object.
lazy_static! {
    static ref GITHUB_CLIENT: Arc<BasicClient> = {
        // Get the global config.
        let config = global_config();

        // Create GitHub OAuth2 client.
        let github_client = BasicClient::new(
            config.github_credentials.client_id.clone(),
            Some(config.github_credentials.client_secret.clone()),
            AuthUrl::new("https://github.com/login/oauth/authorize".into())
                .expect("Invalid GitHub Auth URL"),
            Some(TokenUrl::new("https://github.com/login/oauth/access_token".into())
                .expect("Invalid GitHub Token URL")));
        // Return the client config wrapped in an Arc.
        Arc::new(github_client)
    };
}

impl Oauth2IdentityProvider for GitHubOauth {
    type IdentityType = GitHubIdentity;
    const SERVICE_NAME: &'static str = "github";

    fn get_client() -> Arc<BasicClient> {
        GITHUB_CLIENT.clone()
    }

    fn scopes() -> Vec<Scope> {
        vec![
            // Scope to read user's public profile information.
            Scope::new("read:user".into()),
            // Scope to read user's email address.
            //Scope::new("user:email".into()),
        ]
    }
}

impl Oauth2Identity for GitHubIdentity {
    const USER_ACCOUNT_TY: UserAccountType = UserAccountType::GitHub;

    fn from_basic_token(token: &BasicTokenResponse) -> Self {
        Self { access_token: token.access_token().clone() }
    }

    fn platform_user_id(&self) -> LocalBoxFuture<Result<String, TelescopeError>> {
        Box::pin(async move { self.get_user_id().await })
    }

    fn into_root(self) -> RootIdentity {
        RootIdentity::GitHub(self)
    }

    fn add_to_cookie(self, cookie: &mut AuthenticationCookie) {
        cookie.github = Some(self);
    }
}

impl GitHubIdentity {
    /// Get the github account id of the user associated with this access token.
    /// Note that this is the GitHub GraphQL node ID, and is only compatible with the
    /// GitHub V4 API.
    pub async fn get_user_id(&self) -> Result<String, TelescopeError> {
        // Get the authenticated user and convert their id to a string.
        self.get_authenticated_user()
            .await
            .map(|u| u.id.to_string())
    }

    /// Get the authenticated GitHub user.
    pub async fn get_authenticated_user(&self) -> Result<AuthenticatedUserViewer, TelescopeError> {
        // Query the GitHub GraphQL API.
        github::send_query::<AuthenticatedUser>(&self.access_token, Variables {})
            // Wait for the response
            .await
            // Get the viewer from the response
            .map(|response| response.viewer)
    }

    /// Get the RCOS username of the authenticated user via their GitHub account on the central
    /// RCOS API.
    pub async fn get_rcos_username(&self) -> Result<Option<String>, TelescopeError> {
        // Get the on platform id of this user.
        let platform_id: String = self.get_user_id().await?;
        // Build the variables for a reverse lookup query to the central RCOS API.
        let query_variables = ReverseLookup::make_vars(UserAccountType::GitHub, platform_id);
        // Send the query to the central RCOS API and await response.
        return rcos::send_query::<ReverseLookup>(query_variables)
            .await
            .map(|response| response.username());
    }
}
