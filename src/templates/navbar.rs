use crate::{
    templates::navbar::login_button::LoginButton,
    web::{cookies, RequestContext, Template},
};

mod items;
mod login_button;
use items::*;

/// An adapter type for items in the navbar.
#[derive(Clone, Debug, Serialize, Default)]
pub struct NavbarItem {
    /// The code placed in the navbar.
    navbar_inner: String,
    /// The code (if any) that needs to be placed in the page body.
    body_inner: String,
    /// Whether this item is active.
    active: bool,
}

impl NavbarItem {
    /// Constructor
    fn new(navbar_inner: String, body_inner: impl Into<String>, active: bool) -> Self {
        Self {
            navbar_inner,
            body_inner: body_inner.into(),
            active,
        }
    }
}

/// A navbar definition.
#[derive(Clone, Debug, Serialize)]
pub struct Navbar {
    left_items: Vec<NavbarItem>,
    right_items: Vec<NavbarItem>,
}

impl Navbar {
    /// Get an empty navbar object.
    const fn empty() -> Self {
        Self {
            left_items: Vec::new(),
            right_items: Vec::new(),
        }
    }

    fn add_left(&mut self, ctx: &RequestContext, item: impl MakeNavItem) -> &mut Self {
        self.left_items.push(item.make(ctx));
        self
    }

    fn add_right(&mut self, ctx: &RequestContext, item: impl MakeNavItem) -> &mut Self {
        self.right_items.push(item.make(ctx));
        self
    }

    /// Navbar with homepage, achievement page, projects, developers and sponsors
    fn with_defaults(ctx: &RequestContext) -> Self {
        let mut r = Self::empty();
        r.add_left(ctx, NavbarLink::new("/", "Home"))
            .add_left(ctx, NavbarLink::new("/projects", "Projects"))
            .add_left(ctx, NavbarLink::new("/developers", "Developers"))
            .add_left(ctx, NavbarLink::new("/sponsors", "Sponsors"));
        return r;
    }

    /// Create a navbar based on the page context.
    pub fn from_context(ctx: &RequestContext) -> Self {
        let mut navbar = Self::with_defaults(ctx);
        if let Some(id) = ctx.identity().identity() {
            // todo: change this use of unwrap into something more robust
            unimplemented!()
        } else {
            navbar
                .add_right(
                    ctx,
                    NavbarModal::new("login", "Login", ctx.render(&LoginButton).unwrap()),
                )
                .add_right(ctx, NavbarLink::new("/sign-up", "Sign Up"));
        }
        return navbar;
    }
}

impl Template for Navbar {
    const TEMPLATE_NAME: &'static str = "navbar/navbar";
}
