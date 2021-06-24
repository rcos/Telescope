//! Models and functions for page data passed with almost every template.

use serde_json::Value;
use crate::api::rcos::users::UserRole;

/// The Open Graph Properties rendered in the meta tags at the top of a page.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenGraphProperties {
    /// The title reported to Open Graph. This should usually match the page title.
    pub title: String,

    /// The Open Graph Type (usually "website", sometimes "profile").
    /// See <https://ogp.me/#types>.
    #[serde(rename = "type")]
    pub kind: String,

    /// The image url reported to Open Graph.
    pub image: String,

    /// The url of the current page reported to Open Graph.
    pub url: String,

    /// Any other fields you may choose to add to the Open Graph meta.
    #[serde(flatten)]
    pub other_fields: Value
}

/// Representation of the state of the navbar, which reflects the state of the
/// user's authentication.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "variant")]
pub enum NavbarState {
    /// There is a user authenticated.
    Authenticated {
        username: String,
        role: UserRole,
        is_admin: bool,
        is_coordinator: bool,
        is_mentor: bool,
        is_student: bool,

        /// The request path.
        req_path: String,
    },

    /// During the process of account creation, there is and authentication cookie not associated
    /// with a user.
    AccountCreation {
        /// The path part of the request.
        req_path: String,
    },

    /// There is no user authenticated.
    Unauthenticated {
        /// The path part of the request.
        req_path: String,
    },
}

/// The data passed to the page template.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page {
    /// The page title that appears in the browser tab.
    pub title: String,

    /// The Open Graph properties to render into meta tags.
    pub og_properties: OpenGraphProperties,

    /// The telescope version.
    pub version: &'static str,

    /// The state of the navbar.
    pub navbar: NavbarState
}

