use handlebars::{Handlebars, RenderError};

/// An item in the top navigation bar.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NavbarItem {
    /// The location to redirect to.
    location: String,
    /// The text of the item. This may not get rendered
    /// (if using google material design font, for example, this ligatures into a single symbol)
    text: String,
}

impl NavbarItem {
    /// Create a navbar item.
    fn new(location: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            text: text.into()
        }
    }
}

/// A navbar definition.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Navbar {
    theme_class: String,
    left_items: Vec<NavbarItem>,
    right_items: Vec<NavbarItem>
}

impl Navbar {
    /// Create a new navbar with the default options.
    pub fn new() -> Self {
        let mut s = Self::default();
        s.add_defaults();
        s
    }

    /// Set the theme class of this navbar.
    pub fn theme(mut self, theme_class: impl Into<String>) -> Self {
        self.set_theme(theme_class);
        self
    }

    /// Set the theming css class of this navbar.
    pub fn set_theme(&mut self, theme_class: impl Into<String>) {
        self.theme_class = theme_class.into();
    }

    /// Add a navbar item to the left side of the navbar.
    pub fn add_left(
        &mut self,
        location: impl Into<String>,
        text: impl Into<String>,
    ) {
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
    pub fn add_right(
        &mut self,
        location: impl Into<String>,
        text: impl Into<String>,
    ) {
        self.right_items.push(NavbarItem::new(location, text))
    }


    /// Add a navbar item to the right side of the navbar using the builder pattern.
    pub fn add_right_builder(
        mut self,
        location: impl Into<String>,
        text: impl Into<String>
    ) -> Self {
        self.add_right(location, text);
        self
    }

    /// Add the default navbar items:
    /// RCOS homepage
    /// Achievements
    /// Projects
    /// Developers
    /// Sponsors
    pub fn add_defaults(&mut self) {
        let items = [
            ("RCOS", "/"),
            ("Achievements", "/achievements"),
            ("Projects", "/projects"),
            ("Developers", "/developers"),
            ("Sponsors", "/sponsors")
        ];
        for (text, path) in &items {
            self.add_left(*path, *text);
        }
    }

    /// Render using the template registry
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("navbar", self)
    }
}