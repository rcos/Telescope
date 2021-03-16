use crate::error::TelescopeError;
use actix_web::{HttpRequest, HttpResponse};
use crate::web::services::auth::IdentityProvider;
use std::future::Future;
use actix_web::body::Body;
use futures::future::LocalBoxFuture;

/// Zero-Sized struct representing the RPI CAS identity provider
pub struct RpiCas;

impl IdentityProvider for RpiCas {
    type Client = ();

    fn get_client() -> Self::Client {
        unimplemented!()
    }

    const SERVICE_NAME: &'static str = "rpi_cas";
    type LoginFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type LoginAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;
    type RegistrationAuthenticatedFut = LocalBoxFuture<'static, Result<HttpResponse, TelescopeError>>;

    fn login_handler(req: HttpRequest) -> Self::LoginFut {
        unimplemented!()
    }

    fn registration_handler(req: HttpRequest) -> Self::RegistrationFut {
        unimplemented!()
    }

    fn login_authenticated_handler(req: HttpRequest) -> Self::LoginAuthenticatedFut {
        unimplemented!()
    }

    fn registration_authenticated_handler(req: HttpRequest) -> Self::RegistrationAuthenticatedFut {
        unimplemented!()
    }
}