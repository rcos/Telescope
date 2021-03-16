use crate::error::TelescopeError;
use crate::web::services::auth::{
    identity::Identity,
    IdentityProvider,
    make_redirect_url
};
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;
use actix_web::http::header::LOCATION;
use futures::future::{ready, Ready};

/// The URL of the RPI CAS server.
const RPI_CAS_ENDPOINT: &'static str = "https://cas-auth.rpi.edu/cas";

/// Make the url to redirect users to when authenticating.
fn make_authentication_url(req: &HttpRequest, redir_path: String) -> String {
    // Make the redirect url
    let redirect_url = make_redirect_url(&req, redir_path);

    // Url-encode the redirect url in service parameter.
    let encoded: String = serde_urlencoded::to_string(&[
        ("service", redirect_url.as_str())
    ]).expect("Could not URL-encode CAS parameters.");

    // Build the CAS URL.
    return format!("{}?{}", RPI_CAS_ENDPOINT, encoded);
}

/// Zero-Sized struct representing the RPI CAS identity provider
pub struct RpiCas;

impl IdentityProvider for RpiCas {
    const SERVICE_NAME: &'static str = "rpi_cas";
    type LoginResponse = HttpResponse;
    type RegistrationResponse = HttpResponse;
    type LinkResponse = Result<HttpResponse, TelescopeError>;

    type LoginFut = Ready<Self::LoginResponse>;
    type RegistrationFut = Ready<Self::RegistrationResponse>;
    type LinkFut = LocalBoxFuture<'static, Self::LinkResponse>;

    type LoginAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LinkAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    fn login_handler(req: HttpRequest) -> Self::LoginFut {
        ready(HttpResponse::Found()
            .header(LOCATION,make_authentication_url(&req, Self::login_redirect_path()))
            .finish())
    }

    fn registration_handler(req: HttpRequest) -> Self::RegistrationFut {
        ready(HttpResponse::Found()
            .header(LOCATION, make_authentication_url(&req, Self::registration_redirect_path()))
            .finish())
    }

    fn link_handler(req: HttpRequest, ident: Identity) -> Self::LinkFut {
        return Box::pin(async move {
            // The user must already be authenticated to link RPI CAS to an
            // existing account.
            if ident.identity().await.is_some() {
                // If authenticated make the URL and direct the user there.
                let auth_url = make_authentication_url(&req, Self::link_redirect_path());

                Ok(HttpResponse::Found()
                    .header(LOCATION, auth_url)
                    .finish())
            } else {
                // If not authenticated, return an error
                Err(TelescopeError::NotAuthenticated)
            }
        });
    }

    fn login_authenticated_handler(req: HttpRequest) -> Self::LoginAuthenticatedFut {
        return Box::pin(async move {
            Err(TelescopeError::NotImplemented)
        });
    }

    fn registration_authenticated_handler(req: HttpRequest) -> Self::RegistrationAuthenticatedFut {
        return Box::pin(async move {
            Err(TelescopeError::NotImplemented)
        });
    }

    fn linking_authenticated_handler(
        req: HttpRequest,
        ident: Identity,
    ) -> Self::LinkAuthenticatedFut {
        return Box::pin(async move {
            Err(TelescopeError::NotImplemented)
        });
    }
}
