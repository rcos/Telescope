use crate::env::global_config;
use crate::web::services::auth::oauth2_providers::Oauth2IdentityProvider;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AuthUrl, AuthorizationRequest, Scope, TokenUrl, TokenResponse, AccessToken};
use std::sync::Arc;
use crate::web::services::auth::identity::IdentityCookie;
use crate::web::api::rcos::users::UserAccountType;
use crate::web::telescope_ua;
use hubcaps::{Github, Credentials};
use crate::error::TelescopeError;
use hubcaps::users::AuthenticatedUser;

/// Zero sized type representing the GitHub OAuth2 identity provider.
pub struct GitHubOauth;

/// The identity object stored in the user's cookies for users signed in via
/// GitHub.
#[derive(Serialize, Deserialize)]
pub struct GitHubIdentity {
    /// The OAuth2 Access token granted by GitHub.
    pub access_token: AccessToken
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

    fn make_identity(token_response: &BasicTokenResponse) -> IdentityCookie {
        // Extract the identity and build the identity cookie.
        IdentityCookie::Github(GitHubIdentity {
            access_token: token_response.access_token().clone()
        })
    }
}

impl GitHubIdentity {
    /// Get the authenticated user for this access token.
    async fn get_authenticated_user(&self) -> Result<AuthenticatedUser, TelescopeError> {
        // Get a reference to the string storing the telescope user agent.
        let agent: &str = telescope_ua();

        // Create a github API client via hubcaps.
        let github_client = Github::new(agent, Credentials::Token(self.access_token.secret().clone()))?;

        // Get the authenticated user.
        return github_client.users().authenticated().await.map_err(TelescopeError::from);
    }

    /// Get the github account id of the user associated with this access token.
    pub async fn get_user_id(&self) -> Result<String, TelescopeError> {
        // Get the authenticated user and convert their id to a string.
        self.get_authenticated_user().await.map(|u| u.id.to_string())
    }
}
