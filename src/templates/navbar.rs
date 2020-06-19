use crate::web::{PageContext, Template};
use crate::templates::navbar::login_button::LoginButton;

mod login_button;
mod items;
use items::*;

/// An adapter type for items in the navbar.
#[derive(Clone, Debug, Serialize, Default)]
pub struct NavbarItem {
    /// The code placed in the navbar.
    navbar_inner: String,
    /// The code (if any) that needs to be placed in the page body.
    body_inner: String
}

impl NavbarItem {
    /// Constructor
    fn new(navbar_inner: String, body_inner: impl Into<String>) -> Self {
        Self {navbar_inner, body_inner: body_inner.into()}
    }
}

/// A navbar definition.
#[derive(Clone, Debug, Serialize)]
pub struct Navbar {
    items: Vec<NavbarItem>,
}

impl Navbar {
    /// Get an empty navbar object.
    const fn empty() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Add a navbar item to the navbar.
    /// Assume items are added left to right.
    fn add(&mut self, pc: &PageContext, item: impl MakeNavItem) -> &mut Self {
        self.items.push(item.make(pc));
        self
    }

    fn add_builder(mut self, pc: &PageContext, item: impl MakeNavItem) -> Self {
        self.add(pc, item);
        self
    }

    /// Navbar with homepage, achievement page, projects, developers and sponsors
    fn with_defaults(pc: &PageContext) -> Self {
        Self::empty()
            .add_builder(pc, NavbarLink::new("/", "RCOS"))
            .add_builder(pc, NavbarLink::new("/projects", "Projects"))
            .add_builder(pc, NavbarLink::new("/developers", "Developers"))
            .add_builder(pc, NavbarLink::new("/sponsors", "Sponsors"))
    }

    /// Create a navbar based on the page context.
    pub fn from_context(pc: &PageContext) -> Self {
        let mut navbar = Self::with_defaults(pc);
        if let Some(auth_token) = pc.session().get::<String>("auth_token").unwrap() {
            // todo: change this use of unwrap into something more robust
            unimplemented!()
        } else {
            navbar
                .add(
                    pc,
                    NavbarModal::new(
                        "login",
                        "Login",
                        pc.render(&LoginButton).unwrap()
                    ).right()
                )
                .add(pc, NavbarLink::new("/sign-up", "Sign Up").right());
        }
        return navbar;
    }

}

impl Template for Navbar {
    const TEMPLATE_NAME: &'static str = "navbar/navbar";
}
