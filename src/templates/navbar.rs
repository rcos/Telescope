//! Navbar template constants and functions.

use crate::error::TelescopeError;
use crate::templates::Template;
use crate::web::services::auth::identity::{AuthenticatedIdentities, Identity};
use actix_web::FromRequest;
use actix_web::HttpRequest;

/// The handlebars key for the links on the left side of the navbar.
pub const LEFT_ITEMS: &'static str = "left_items";

/// The handlebars key for the links on the right side of the navbar.
pub const RIGHT_ITEMS: &'static str = "right_items";

/// The handlebars key for denoting whether an item on the navbar is active.
pub const IS_ACTIVE: &'static str = "is_active";

/// The handlebars key for the URL of a navbar link.
pub const LOCATION: &'static str = "location";

/// The handlebars key for the CSS class attributes of a link on the navbar.
pub const CLASS: &'static str = "class";

/// The handlebars key for the text inside a link on the navbar.
pub const TEXT: &'static str = "text";

/// The handlebars key for an icon next to the text on an item in the navbar.
pub const ICON: &'static str = "icon";

/// Create an empty navbar template with a reference to the navbar handlebars
/// file.
fn empty_navbar() -> Template {
    Template::new("navbar/navbar")
}

/// Create a navbar item with the given text and location.
fn item(req_path: &str, text: impl Into<String>, location: impl Into<String>) -> Template {
    let loc_str: String = location.into();
    Template::new("navbar/item")
        .field(TEXT, text.into())
        .field(LOCATION, loc_str.as_str())
        .field(IS_ACTIVE, req_path == loc_str.as_str())
        .field(CLASS, "nav-link")
}

/// Create an empty navbar and add the default links that should be visible to
/// everyone at all times.
fn with_defaults(req_path: &str) -> Template {
    let left_items = vec![
        item(req_path, "Home", "/"),
        // item(req_path, "Projects", "/projects"),
        // item(req_path, "Developers", "/developers"),
        item(req_path, "Sponsors", "/sponsors"),
        // item(req_path, "Blog", "/blog"),
        // item(req_path, "Calendar", "/calendar")
    ];

    // Add items to empty navbar.
    empty_navbar().field(LEFT_ITEMS, left_items)
}

/// Construct a navbar for an anonymous viewer by adding onto the defaults.
fn userless(req_path: &str) -> Template {
    let right_items = vec![
        item(req_path, "Sign Up", "/register").field(CLASS, "btn mr-2 mb-2 btn-primary"),
        item(req_path, "Sign In", "/login").field(CLASS, "btn mr-2 mb-2 btn-primary"),
    ];

    // Add items to right side of navbar.
    with_defaults(req_path).field(RIGHT_ITEMS, right_items)
}

/// Construct a navbar for a given username
fn for_user(req_path: &str, username: &str) -> Template {
    let right_items = vec![
        item(req_path, "Profile", format!("/users/{}", username))
            .field(CLASS, "btn mr-2 mb-2 btn-primary"),
        item(req_path, "Logout", "/logout").field(CLASS, "btn mr-2 mb-2 btn-secondary"),
    ];

    // Add items to right side of navbar
    with_defaults(req_path).field(RIGHT_ITEMS, right_items)
}

/// Create a navbar template for
pub async fn for_request(req: &HttpRequest) -> Result<Template, TelescopeError> {
    // Extract the authenticated identities from the request.
    let identity: Option<AuthenticatedIdentities> = Identity::extract(req).await?.identity().await;

    // If the user is authenticated.
    if let Some(authenticated) = identity {
        // Check if there is an authenticated RCOS account
        if let Some(username) = authenticated.get_rcos_username().await? {
            // If there is make a navbar with the username.
            return Ok(for_user(req.path(), username.as_str()));
        }
    }

    // If the user is not authenticated or there is no username, return a user-less navbar.
    return Ok(userless(req.path()));
}
