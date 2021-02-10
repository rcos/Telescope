//! Module with definitions for several identity providers that can be used to
//! register a new account or sign in.

/// A trait to be implemented by types that represent identity providers.
/// This includes GitHub Oauth, RPI CAS, etc.
pub trait IdentityProvider {
    type Info;

    fn get_auth_url();
}
