use crate::web::services::auth::oauth2_providers::Oauth2IdentityProvider;
use std::sync::Arc;
use oauth2::{AuthorizationRequest, AuthUrl, TokenUrl, Scope};
use oauth2::basic::BasicClient;
use crate::env::global_config;

/// Zero sized typ representing the GitHub OAuth2 identity provider.
pub struct GitHubOauth;

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

    fn add_scopes(auth_req: AuthorizationRequest) -> AuthorizationRequest {
        auth_req
            // Scope to read user's public profile information.
            .add_scope(Scope::new("read:user".into()))
            // Scope to read user's email address.
            .add_scope(Scope::new("user:email".into()))
    }
}
