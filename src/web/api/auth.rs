//! Authentication for the central RCOS API.

use actix_web::client::Client;
use std::sync::Arc;
use crate::env::{ConcreteConfig, global_config};
use actix_web::http::header::ACCEPT;
use jsonwebtoken::{Header, EncodingKey, encode};

/// The database role name for authenticated requests.
const AUTHENTICATED_USER: &'static str = "api_user";

/// The database role name for unauthenticated requests.
const ANONYMOUS_USER: &'static str = "web_anon";

/// JWT Claims used to authenticate with the central RCOS API.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ApiJwtClaims {
    /// The role to use when accessing the API. For authenticated requests,
    /// this will always be "api_user". Otherwise, the default is "web_anon".
    role: &'static str,
}

/// Create an HTTP client with the given data as the JWT role claim.
fn client(role: &'static str) -> Client {
    // Get the global config.
    let config: Arc<ConcreteConfig> = global_config();
    // Get the JWT secret from the config.
    let jwt_secret: &[u8] = config.jwt_secret.as_bytes();

    // Encode the JWT.
    let claims: ApiJwtClaims = ApiJwtClaims { role };
    let jwt: String = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret))
        .expect("Could not encode JWT");

    // Construct and return an HTTP client.
    return Client::builder()
        .header(ACCEPT, "application/json")
        .bearer_auth(jwt)
        .finish();
}

/// Create an HTTP client to send unauthenticated API requests.
pub fn unauthenticated_client() -> Client {
    client(ANONYMOUS_USER)
}

/// Create an HTTP client to send authenticated API requests.
pub fn authenticated_client() -> Client {
    client(AUTHENTICATED_USER)
}
