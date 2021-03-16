use crate::error::TelescopeError;
use crate::web::services::auth::identity::Identity;
use crate::web::services::auth::IdentityProvider;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::LocalBoxFuture;

/// The URL of the RPI CAS server.
const RPI_CAS_ENDPOINT: &'static str = "https://cas-auth.rpi.edu/cas";

/// Zero-Sized struct representing the RPI CAS identity provider
pub struct RpiCas;

impl IdentityProvider for RpiCas {
    const SERVICE_NAME: &'static str = "rpi_cas";
    type LoginFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LinkFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LoginAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationAuthenticatedFut =
        LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LinkAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    fn login_handler(req: HttpRequest) -> Self::LoginFut {
        unimplemented!()
    }

    fn registration_handler(req: HttpRequest) -> Self::RegistrationFut {
        unimplemented!()
    }

    fn link_handler(req: HttpRequest, ident: Identity) -> Self::LinkFut {
        unimplemented!()
    }

    fn login_authenticated_handler(req: HttpRequest) -> Self::LoginAuthenticatedFut {
        unimplemented!()
    }

    fn registration_authenticated_handler(req: HttpRequest) -> Self::RegistrationAuthenticatedFut {
        unimplemented!()
    }

    fn linking_authenticated_handler(
        req: HttpRequest,
        ident: Identity,
    ) -> Self::LinkAuthenticatedFut {
        unimplemented!()
    }
}
