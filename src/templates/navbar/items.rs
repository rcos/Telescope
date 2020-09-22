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
    /// CSS classes associated with this link.
    class: String,
}

impl NavbarLink {
    /// Create a new navbar link button (with default styling).
    pub fn new(location: impl Into<String>, text: impl Into<String>) -> Self {
        let loc = location.into();
        Self {
            location: loc.clone(),
            text: text.into(),
            is_root: loc == "/",
            class: "nav-link".to_string()
        }
    }

    /// Change the CSS classes of this item.
    /// Follows the builder pattern.
    ///
    /// Default value is 'nav-link'.
    pub fn class(mut self, new_class: impl Into<String>) -> Self {
        self.class = new_class.into();
        self
    }
}

impl MakeNavItem for NavbarLink {
    /// Adapt a navbar link into a navbar item.
    fn make(&self, pc: &RequestContext) -> NavbarItem {
        let render = self.clone();
        // if the webpage path is the same as the nav item location,
        // focus on that nav item.
        let path = render.location.as_str();
        let focus = pc.request().path() == render.location;
        // render.focus = focus;
        NavbarItem::new(pc.render(&render), "", focus)
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
    /// The Bootstrap button class
    button_class: String
}

impl NavbarModal {
    /// Create a new modal navbar item.
    pub fn new(
        id: impl Into<String>,
        header: impl Into<String>,
        class: impl Into<String>,
        inner: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            text: header.into(),
            inner: inner.into(),
            button_class: class.into()
        }
    }
}

impl MakeNavItem for NavbarModal {
    /// Adapt a navbar modal into a navbar item.
    fn make(&self, pc: &RequestContext) -> NavbarItem {
        NavbarItem::new(
            pc.handlebars().render("navbar/modal-button", self).unwrap(),
            pc.handlebars().render("navbar/modal-body", self).unwrap(),
            false,
        )
    }
}

