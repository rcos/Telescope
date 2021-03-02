//! Authentication for the central RCOS API.

use crate::env::{global_config, ConcreteConfig};
use actix_web::client::Client;
use actix_web::http::header::{ACCEPT, CONTENT_TYPE};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

/// The database role name for authenticated requests.
const AUTHENTICATED_USER: &'static str = "admin";

/// Accept responses in JSON format.
const JSON_MIME: &'static str = "application/json";

/// The issuer claim in JWTs issued by telescope.
const JWT_ISSUER: &'static str = "telescope";

/// JWT Claims used to authenticate with the central RCOS API.
#[derive(Serialize, Clone, Debug)]
struct ApiJwtClaims {
    /// Who issued the JWY token. This should always be "telescope".
    iss: &'static str,
    /// The subject of the JWT. This is the user that the claim is for.
    /// May be none.
    sub: Option<String>,
    /// When the claim was issued. This should be the UNIX time
    /// (number of seconds since the epoch) when the JWT was issued.
    iat: i64,
    /// Claims required by hasura.
    #[serde(rename = "https://hasura.io/jwt/claims")]
    hasura_claims: HasuraJwtClaims,
}

/// JWT Claims in the Hasura namespace.
#[derive(Serialize, Clone, Debug)]
struct HasuraJwtClaims {
    /// The roles this user is allowed to use.
    #[serde(rename = "x-hasura-allowed-roles")]
    allowed_roles: Vec<&'static str>,
    /// The role that this user should use if no different one is passed in the
    /// `x-hasura-role` request header.
    #[serde(rename = "x-hasura-default-role")]
    default_role: &'static str,
    /// The user ID. This should match the one in the top level of the JWT token.
    #[serde(rename = "x-hasura-user-id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
}

impl ApiJwtClaims {
    /// Construct and sign a new JWT.
    fn new(subject: Option<String>) -> String {
        // Get the global config.
        let config: Arc<ConcreteConfig> = global_config();
        // Get the JWT secret from the config.
        let jwt_secret: &[u8] = config.jwt_secret.as_bytes();

        // Create the JWT.
        let jwt = ApiJwtClaims {
            iss: JWT_ISSUER,
            sub: subject.clone(),
            iat: Utc::now().timestamp(),
            hasura_claims: HasuraJwtClaims {
                default_role: AUTHENTICATED_USER,
                allowed_roles: vec![AUTHENTICATED_USER],
                user_id: subject,
            },
        };

        // Encode and return the JWT
        return encode(
            &Header::default(),
            &jwt,
            &EncodingKey::from_secret(jwt_secret),
        )
        .expect("Could not encode JWT");
    }
}

/// Create an HTTP client with an optional subject claim in the JWT. This will have the proper
/// Accept and Content-Type headers to send and receive GraphQL data.
pub fn make_api_client(subject: Option<String>) -> Client {
    // Make JWT.
    let jwt: String = ApiJwtClaims::new(subject);

    // Construct and return an HTTP client.
    return Client::builder()
        .header(ACCEPT, JSON_MIME)
        .header(CONTENT_TYPE, JSON_MIME)
        .bearer_auth(jwt)
        .finish();
}
