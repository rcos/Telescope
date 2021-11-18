//! CAS authentication is implemented roughly following the diagram
//! [here](https://apereo.github.io/cas/4.2.x/protocol/CAS-Protocol.html)
//! and work from RPI students who came before me.

use crate::api::rcos::users::accounts::link::LinkUserAccount;
use crate::api::rcos::users::accounts::lookup::AccountLookup;
use crate::api::rcos::users::accounts::reverse_lookup::ReverseLookup;
use crate::api::rcos::users::UserAccountType;
use crate::error::TelescopeError;

use crate::web::services::auth::identity::{AuthenticationCookie, RootIdentity};
use crate::web::services::auth::{identity::Identity, make_redirect_url, IdentityProvider};
use actix_web::http::header::LOCATION;
use actix_web::{web::Query, FromRequest};
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use futures::future::{ready, Ready};
use regex::Regex;
use uuid::Uuid;

/// The URL of the RPI CAS server.
const RPI_CAS_ENDPOINT: &'static str = "https://cas.auth.rpi.edu/cas";

/// Query parameters of the request sent to Telescope after
/// the user is authenticated with RPI CAS.
#[derive(Deserialize, Clone, Debug)]
struct CasAuthenticatedParameters {
    ticket: String,
}

/// Query parameters sent in request to the CAS endpoint by telescope after
/// the user has authenticated.
#[derive(Serialize, Clone, Debug)]
struct CasIdentificationParameters {
    ticket: String,
    service: String,
}

/// Function to make CAS user regular expression.
fn make_cas_user_regex() -> Regex {
    // Don't validate the RPI CAS XML too much. Just look for the user tag
    // in the cas namespace and capture its value.
    Regex::new(r"<cas:user>([[:alnum:]]+)</cas:user>").expect("Could not create CAS RCS ID Regex")
}

lazy_static! {
    static ref CAS_USER_REGEX: Regex = make_cas_user_regex();
}

/// Use the CAS user id regular expression to extract the RCS ID of a user from
/// the XML returned by the CAS service.
fn extract_rcs_id(xml: &str) -> Option<String> {
    Some(
        CAS_USER_REGEX
            .captures(xml)?
            // The first capture should be the RCS ID.
            .get(1)?
            // Get the string
            .as_str()
            // Convert to lowercase.
            .to_lowercase(),
    )
}

/// The RPI CAS based identity object just stores the user's RCS id directly.
/// We do not get any other information from the RPI CAS service and the user's
/// RCS id should never change.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpiCasIdentity {
    /// The authenticated RCS ID of the user with this cookie.
    pub rcs_id: String,
}

impl RpiCasIdentity {
    /// Get the RCOS user ID (if one exists) associated with this RCS ID.
    pub async fn get_rcos_user_id(&self) -> Result<Option<Uuid>, TelescopeError> {
        ReverseLookup::execute(UserAccountType::Rpi, self.rcs_id.clone()).await
    }
}

/// After the user has authenticated with CAS it will send them back to telescope
/// with a service ticket. This function will extract the service ticket and
/// use it to access the user's information via CAS. On success, this function return's the
/// user's RCS ID as a string (in lowercase).
async fn cas_authenticated(
    req: &HttpRequest,
    redir_path: String,
) -> Result<String, TelescopeError> {
    // Extract the CAS parameters from the query
    let Query(params): Query<CasAuthenticatedParameters> =
        Query::<CasAuthenticatedParameters>::extract(req)
            .await
            // Convert any errors that occur.
            .map_err(|err| {
                // Log the error
                error!(
                    "Could not extract CAS ticket from request parameters: {}",
                    err
                );

                // Display an error message to the user.
                TelescopeError::BadRequest {
                    header: "Malformed CAS request".into(),
                    message: format!(
                        "The RPI CAS endpoint did not respond with the appropriate \
                    data. Please try again. If this error persists, contact a coordinator and file \
                    an issue on Telescope's GitHub. Internal error: {}",
                        err
                    ),
                    show_status_code: true,
                }
            })?;

    // Make the query parameters to send to the CAS validation server
    let validation_params = CasIdentificationParameters {
        // Get the URL that the user made the request to without any
        // path or parameters.
        service: make_redirect_url(req, redir_path).to_string(),
        ticket: params.ticket,
    };
    // Url-encode validation query
    let validation_query: String = serde_urlencoded::to_string(validation_params)
        .expect("Could not URL-encode CAS validation parameters");
    // Build the endpoint to query for user info.
    let validation_url: String =
        format!("{}/serviceValidate?{}", RPI_CAS_ENDPOINT, validation_query);

    // Send request to CAS service and wait for response.
    let cas_xml: String = reqwest::get(validation_url.as_str())
        .await
        .map_err(TelescopeError::rpi_cas_error)?
        .text()
        .await
        .map_err(TelescopeError::rpi_cas_error)?;

    // Extract and return the RCS id.
    return extract_rcs_id(cas_xml.as_str()).ok_or(TelescopeError::RpiCasError(format!(
        "Could not extract RCS ID from RPI CAS response. Response xml: {}",
        cas_xml
    )));
}

/// Make the url to redirect users to when authenticating.
fn make_authentication_url(req: &HttpRequest, redir_path: String) -> String {
    // Make the redirect url
    let redirect_url = make_redirect_url(&req, redir_path);

    // Url-encode the redirect url in service parameter.
    let encoded: String = serde_urlencoded::to_string(&[("service", redirect_url.as_str())])
        .expect("Could not URL-encode CAS parameters.");

    // Build the CAS URL.
    return format!("{}/login?{}", RPI_CAS_ENDPOINT, encoded);
}

/// Zero-Sized struct representing the RPI CAS identity provider
pub struct RpiCas;

impl IdentityProvider for RpiCas {
    const SERVICE_NAME: &'static str = "rpi_cas";
    const USER_ACCOUNT_TY: UserAccountType = UserAccountType::Rpi;

    type LoginResponse = HttpResponse;
    type RegistrationResponse = HttpResponse;
    type LinkResponse = Result<HttpResponse, TelescopeError>;

    type LoginFut = Ready<Self::LoginResponse>;
    type RegistrationFut = Ready<Self::RegistrationResponse>;
    type LinkFut = LocalBoxFuture<'static, Self::LinkResponse>;

    type LoginAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationAuthenticatedFut =
        LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LinkAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    fn login_handler(req: HttpRequest) -> Self::LoginFut {
        ready(
            HttpResponse::Found()
                .header(
                    LOCATION,
                    make_authentication_url(&req, Self::login_redirect_path()),
                )
                .finish(),
        )
    }

    fn registration_handler(req: HttpRequest) -> Self::RegistrationFut {
        ready(
            HttpResponse::Found()
                .header(
                    LOCATION,
                    make_authentication_url(&req, Self::registration_redirect_path()),
                )
                .finish(),
        )
    }

    fn link_handler(req: HttpRequest, ident: Identity) -> Self::LinkFut {
        return Box::pin(async move {
            // The user must already be authenticated to link RPI CAS to an
            // existing account.
            if let Some(authenticated_identity) = ident.identity().await {
                // Make sure they are authenticated on a different platform.
                if let RootIdentity::RpiCas(_) = authenticated_identity.root {
                    return Err(TelescopeError::BadRequest {
                        header: "RPI CAS already linked".into(),
                        message: "You are already signed into an RPI CAS account.".into(),
                        show_status_code: false,
                    });
                }

                // If authenticated make the URL and direct the user there.
                let auth_url = make_authentication_url(&req, Self::link_redirect_path());

                Ok(HttpResponse::Found().header(LOCATION, auth_url).finish())
            } else {
                // If not authenticated, return an error
                Err(TelescopeError::NotAuthenticated)
            }
        });
    }

    fn login_authenticated_handler(req: HttpRequest) -> Self::LoginAuthenticatedFut {
        return Box::pin(async move {
            // Get the RCS ID of the user logging in.
            let rcs_id: String = cas_authenticated(&req, Self::login_redirect_path()).await?;
            let token = RpiCasIdentity { rcs_id };
            // Get the RCOS user ID of the account linked to this RCS id.
            let user_id = token
                .get_rcos_user_id()
                .await?
                // Throw error on missing user account
                .ok_or(TelescopeError::resource_not_found(
                    "Could not find associated user account",
                    format!(
                        "Could not find a Telescope account for {}@rpi.edu. Please \
                    create an account or login using another provider.",
                        token.rcs_id
                    ),
                ))?;

            // Set the user's identity cookie
            let identity: Identity = Identity::extract(&req).await?;
            identity.save(&RootIdentity::RpiCas(token).make_authenticated_cookie());
            // Redirect the user to their profile.
            Ok(HttpResponse::Found()
                .header(LOCATION, format!("/user/{}", user_id))
                .finish())
        });
    }

    fn registration_authenticated_handler(req: HttpRequest) -> Self::RegistrationAuthenticatedFut {
        return Box::pin(async move {
            // Authenticate with the RPI CAS service and extract the user's RCS ID.
            let rcs_id: String =
                cas_authenticated(&req, Self::registration_redirect_path()).await?;
            // Put the RCS ID in an identity cookie.
            let cookie: RootIdentity = RootIdentity::RpiCas(RpiCasIdentity { rcs_id });
            // Give the cookie to the user
            let identity: Identity = Identity::extract(&req).await?;
            identity.save(&cookie.make_authenticated_cookie());
            // Redirect the user to complete registration
            Ok(HttpResponse::Found()
                .header(LOCATION, "/register/finish")
                .finish())
        });
    }

    fn linking_authenticated_handler(
        req: HttpRequest,
        ident: Identity,
    ) -> Self::LinkAuthenticatedFut {
        return Box::pin(async move {
            // Get the authenticated identities of this user.
            let authenticated: AuthenticationCookie = ident
                .identity()
                .await
                .ok_or(TelescopeError::NotAuthenticated)?;

            // Get the RCOS user ID of the authenticated user.
            let user_id = authenticated.get_user_id_or_error().await?;

            // Get the RCS ID of the authenticated user (if one exists).
            let existing_rcs_id: Option<String> =
                AccountLookup::send(user_id, Self::USER_ACCOUNT_TY).await?;

            // Get the RCS ID from the authenticated RPI CAS response.
            let new_rcs_id: String = cas_authenticated(&req, Self::link_redirect_path()).await?;

            // We add the new RCS ID to the database for any user who doesn't have one.
            let add_new_to_db: bool = existing_rcs_id.is_none();
            // Also check if the new one matches the existing one.
            let new_matches_existing: bool = existing_rcs_id
                .as_ref()
                .map(|existing| existing.as_str() == new_rcs_id.as_str())
                .unwrap_or(true);

            // Add to database if needed.
            if add_new_to_db {
                // Link the account.
                LinkUserAccount::send(user_id, Self::USER_ACCOUNT_TY, new_rcs_id.clone()).await?;
            }

            // Throw an error if the new RCS ID doesn't match the linked one.
            if !new_matches_existing {
                return Err(TelescopeError::BadRequest {
                    header: "Different RCS ID already linked".into(),
                    message: format!(
                        "This account is already linked to the RPI CAS system \
                        as {}@rpi.edu. Please unlink this RCS id before linking a different one. \
                        If you did not link this account please contact a coordinator.\
                        ",
                        existing_rcs_id.unwrap()
                    ),
                    show_status_code: false,
                });
            }

            // We are all set at this point, redirect to the user's account.
            return Ok(HttpResponse::Found()
                .header(LOCATION, format!("/user/{}", user_id))
                .finish());
        });
    }
}
