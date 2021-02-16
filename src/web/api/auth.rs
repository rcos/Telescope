//! Authentication for the central RCOS API.

use actix_web::client::Client;
use std::sync::Arc;
use crate::env::{ConcreteConfig, global_config};
use actix_web::http::header::ACCEPT;
use jsonwebtoken::{Header, EncodingKey, encode};

/// The database role name for authenticated requests.
pub const AUTHENTICATED_USER: &'static str = "api_user";

/// The database role name for unauthenticated requests.
pub const ANONYMOUS_USER: &'static str = "web_anon";

/// Accept responses in JSON format.
pub const ACCEPT_JSON: &'static str = "application/json";

/// Accept responses in CSV format.
pub const ACCEPT_CSV: &'static str = "text/csv";

/// JWT Claims used to authenticate with the central RCOS API.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ApiJwtClaims {
    /// The role to use when accessing the API. For authenticated requests,
    /// this will always be "api_user". Otherwise, the default is "web_anon".
    role: &'static str,
}

/// Create an HTTP client with the given data as the JWT role claim.
pub fn make_client(role: &'static str, accept: &'static str) -> Client {
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
        .header(ACCEPT, accept)
        .bearer_auth(jwt)
        .finish();
}
