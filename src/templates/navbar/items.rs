use crate::templates::navbar::NavbarItem;
use crate::web::{RequestContext, Template};

pub trait MakeNavItem {
    /// Turn something into a navbar item.
    fn make(&self, pc: &RequestContext) -> NavbarItem;
}

/// A button that just links to a another part of the site (or another site entirely.)
/// This is good for most items on the header bar.
#[derive(Clone, Debug, Serialize)]
pub struct NavbarLink {
    is_root: bool,
    /// The location to redirect to.
    location: String,
    /// The text of the item. This may not get rendered
    /// (if using google material design font, for example, this ligatures into a single symbol)
    text: String,
}

impl NavbarLink {
    /// Create a new navbar link button.
    pub fn new(location: impl Into<String>, text: impl Into<String>) -> Self {
        let loc = location.into();
        Self {
            location: loc.clone(),
            text: text.into(),
            is_root: loc == "/",
        }
    }
}

impl MakeNavItem for NavbarLink {
    /// Adapt a navbar link into a navbar item.
    fn make(&self, pc: &RequestContext) -> NavbarItem {
        let mut render = self.clone();
        // if the webpage path starts with the nav item location, focus on that nav item.
        let path = &render.location[1..];
        let focus = pc.request().path().starts_with(path);
        // render.focus = focus;
        NavbarItem::new(pc.render(&render).unwrap(), "", focus)
    }
}

impl Template for NavbarLink {
    const TEMPLATE_NAME: &'static str = "navbar/link";
}

/// A button that opens a modal. Used only for the login button currently.
#[derive(Clone, Debug, Serialize)]
pub struct NavbarModal {
    /// The id in the html page of the modal.
    id: String,
    /// The html of the modal dialogue.
    inner: String,
    /// The header of the modal dialogue, and the text of the button.
    text: String,
}

impl NavbarModal {
    /// Create a new modal navbar item.
    pub fn new(id: impl Into<String>, header: impl Into<String>, inner: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: header.into(),
            inner: inner.into(),
        }
    }
}

impl MakeNavItem for NavbarModal {
    /// Adapt a navbar modal into a navbar item.
    fn make(&self, pc: &RequestContext) -> NavbarItem {
        NavbarItem::new(
            pc.handlebars().render("navbar/modal-button", self).unwrap(),
            pc.handlebars().render("navbar/modal-body", self).unwrap(),
            false
        )
    }
}
