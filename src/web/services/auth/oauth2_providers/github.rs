use crate::env::global_config;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::{AuthenticatedIdentities, AuthenticatedIdentity};
use crate::web::services::auth::oauth2_providers::Oauth2IdentityProvider;
use hubcaps::{Credentials, Github};
use crate::web::api::{
    rcos,
    github::{
        self,
        users::{
            authenticated_user::{
                AuthenticatedUser,
                authenticated_user::{
                    AuthenticatedUserViewer,
                    Variables
                }
            },
        },
    }
};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AccessToken, AuthUrl, Scope, TokenResponse, TokenUrl};
use std::sync::Arc;
use crate::web::api::rcos::users::accounts::reverse_lookup::ReverseLookup;
use crate::web::api::rcos::users::UserAccountType;
use crate::web::services::auth::IdentityProvider;
use std::future::Future;
use futures::future::LocalBoxFuture;

/// Zero sized type representing the GitHub OAuth2 identity provider.
pub struct GitHubOauth;

/// The identity object stored in the user's cookies for users signed in via
/// GitHub.
#[derive(Serialize, Deserialize)]
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
    const SERVICE_NAME: &'static str = "github";
    const USER_ACCOUNT_TYPE: UserAccountType = GitHubIdentity::USER_ACCOUNT_TYPE;

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

    fn make_identity(token_response: &BasicTokenResponse) -> AuthenticatedIdentities {
        // Extract the identity and build the identity cookie.
        AuthenticatedIdentities::Github(GitHubIdentity {
            access_token: token_response.access_token().clone(),
        })
    }
}

impl GitHubIdentity {
    /// Get the github account id of the user associated with this access token.
    /// Note that this is the GitHub GraphQL node ID, and is only compatible with the
    /// GitHub V4 API.
    pub async fn get_user_id(&self) -> Result<String, TelescopeError> {
        // Get the authenticated user and convert their id to a string.
        self.get_authenticated_user().await.map(|u| u.id.to_string())
    }
}

impl AuthenticatedIdentity for GitHubIdentity {
    const USER_ACCOUNT_TYPE: UserAccountType = UserAccountType::GitHub;
    type AuthenticatedUser = AuthenticatedUserViewer;
    type AuthenticatedUserFuture = LocalBoxFuture<'static, Result<Self::AuthenticatedUser, TelescopeError>>;

    fn get_authenticated_user(&self) -> Self::AuthenticatedUserFuture {
        // Clone the access token for ownership reasons.
        let access_token: AccessToken = self.access_token.clone();
        // Make and return the boxed future.
        return Box::pin(async move {
            // Query the GitHub GraphQL API.
            github::send_query::<AuthenticatedUser>(&access_token, Variables {})
                // Wait for the response
                .await
                // Get the viewer from the response
                .map(|response| response.viewer)
        });
    }

    fn get_rcos_username(&self) -> LocalBoxFuture<Result<Option<String>, TelescopeError>> {
        // Make and return a boxed future.
        return Box::pin(async move {
            // Get the on platform id of this user.
            let platform_id: String = self.get_user_id().await?;
            // Build the variables for a reverse lookup query to the central RCOS API.
            let query_variables = ReverseLookup::make_vars(Self::USER_ACCOUNT_TYPE, platform_id);
            // Send the query to the central RCOS API and await response (we have no subject for this
            // request since we are requesting something that would be the subject field)
            return rcos::send_query::<ReverseLookup>(None, query_variables)
                .await
                .map(|response| response.username());
        });
    }
}
