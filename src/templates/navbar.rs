use crate::web::{PageContext, Template};

/// An item in the top navigation bar.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NavbarItem {
    /// The location to redirect to.
    location: String,
    /// The text of the item. This may not get rendered
    /// (if using google material design font, for example, this ligatures into a single symbol)
    text: String,
    /// Whether this item of the navbar is focussed
    focus: bool,
    /// Style extras added onto the the element at render time.
    /// This is localized entirely to this module and serves to make the
    /// Homepage button bold.
    style_extras: String,
}

impl NavbarItem {
    /// Create a navbar item.
    fn new(location: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            text: text.into(),
            focus: false,
            style_extras: "".to_owned(),
        }
    }
}

/// A navbar definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

    /// Navbar with homepage, achievement page, projects, developers and sponsors
    fn with_defaults() -> Self {
        let mut s = Self::empty().add_left_builder("/", "RCOS");
        // make homepage button bold.
        s.left_items[0].style_extras = "font-weight:bold;".to_owned();
        // add remaining buttons
        s.add_left_builder("/achievements", "Achievements")
            .add_left_builder("/projects", "Projects")
            .add_left_builder("/developers", "Developers")
            .add_left_builder("/sponsors", "Sponsors")
    }

    /// Create a navbar without a signed in user. This is a default navbar with
    /// "Sign up" and "Login" buttons.
    fn userless() -> Self {
        Self::with_defaults()
            .add_right_builder("/sign-up", "Sign Up")
            .add_right_builder("/login", "Login")
    }

    /// Set the focused item in the navbar.
    /// (This searches through all existing items in the navbar and turns on focus on
    /// the one who's text matches)
    fn set_focus<T>(&mut self, path: T)
    where
        String: PartialEq<T>,
    {
        let iter = self
            .left_items
            .iter_mut()
            .chain(self.right_items.iter_mut());
        for nav_item in iter {
            if nav_item.location == path {
                nav_item.focus = true;
            } else {
                nav_item.focus = false;
            }
        }
    }

    /// Create a navbar based on the page context.
    pub fn from_context(pc: &PageContext) -> Self {
        let mut bar: Navbar =
            if let Some(auth_token) = pc.session().get::<String>("auth_token").unwrap() {
                // todo: change this use of unwrap into something more robust
                unimplemented!()
            } else {
                Self::userless()
            };

        let matches = [
            "achievements",
            "projects",
            "developers",
            "sponsors",
            "login",
            "sign-up",
        ];

        let mut found = false;
        for path in matches.iter() {
            if pc.request().path().starts_with(path) {
                found = true;
                bar.set_focus("/".to_owned() + path);
                break;
            }
        }
        if !found {
            bar.set_focus("/");
        }

        return bar;
    }

    /// Add a navbar item to the left side of the navbar.
    pub fn add_left(&mut self, location: impl Into<String>, text: impl Into<String>) {
        self.left_items.push(NavbarItem::new(location, text))
    }

    /// Add a navbar item to the left side of the navbar using the builder pattern.
    pub fn add_left_builder(
        mut self,
        location: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        self.add_left(location, text);
        self
    }

    /// Add a navbar item to the right side of the navbar.
    pub fn add_right(&mut self, location: impl Into<String>, text: impl Into<String>) {
        self.right_items.push(NavbarItem::new(location, text))
    }

    /// Add a navbar item to the right side of the navbar using the builder pattern.
    pub fn add_right_builder(
        mut self,
        location: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        self.add_right(location, text);
        self
    }
}

impl Template for Navbar {
    const TEMPLATE_NAME: &'static str = "navbar";
}
