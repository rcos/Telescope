pub mod jumbotron;
pub mod navbar;
pub mod page;
pub mod profile;
pub mod login;
pub mod graphql_playground;

/// Re-export everything in the static_pages module publicly.
pub mod static_pages;
pub use static_pages::*;
