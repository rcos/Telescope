//! Navbar template constants and functions.

use crate::templates::Template;
use serde::Serialize;
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
        .field(LEFT_ITEMS, None)
        .field(RIGHT_ITEMS, None)
}

/// Create a navbar item with the given text and location.
fn item(req: &HttpRequest, text: impl Into<String>, location: impl Into<String>) -> Template {
    let loc_str: String = location.into();
    Template::new("navbar/item")
        .field(TEXT, text.into())
        .field(LOCATION, loc_str.as_str())
        .field(IS_ACTIVE, req.uri().path() == loc_str.as_str())
        .field(CLASS, "nav-link")
}

/// Create an empty navbar and add the default links that should be visible to
/// everyone at all times.
fn with_defaults(req: &HttpRequest) -> Template {
    let left_items = vec![
        item(req, "Home", "/"),
        item(req, "Projects", "/projects"),
        item(req, "Developers", "/developers"),
        item(req, "Sponsors", "/sponsors"),
        item(req, "Blog", "/blog")
    ];

    // Add items to empty navbar.
    empty_navbar().field(LEFT_ITEMS, left_items)
}

/// Construct a navbar for an anonymous viewer by adding onto the defaults.
fn userless(req: &HttpRequest) -> Template {
    unimplemented!()
}

