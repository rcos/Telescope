use handlebars::{Handlebars, RenderError};

/// An item in the top navigation bar.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NavbarItem {
    /// The location to redirect to.
    location: String,
    /// The text of the item. This may not get rendered
    /// (if using google material design font, for example, this ligatures into a single symbol)
    text: String,
    /// Any extra styling added to the navbar item.
    styling: String
}

impl NavbarItem {
    /// Create a navbar item.
    fn new(location: impl Into<String>, text: impl Into<String>, styling: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            text: text.into(),
            styling: styling.into()
        }
    }
}

/// A navbar definition.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Navbar {
    left_items: Vec<NavbarItem>,
    right_items: Vec<NavbarItem>
}

impl Navbar {
    /// Create a new navbar with the default options.
    pub fn new() -> Self {
        Self::default()
    }

    // /// Set the theme class of this navbar.
    // pub fn theme(mut self, theme_class: impl Into<String>) -> Self {
    //     self.set_theme(theme_class);
    //     self
    // }
    //
    // /// Set the theming css class of this navbar.
    // pub fn set_theme(&mut self, theme_class: impl Into<String>) {
    //     self.theme_class = theme_class.into();
    // }

    /// Add a navbar item to the left side of the navbar.
    pub fn add_left(
        &mut self,
        location: impl Into<String>,
        text: impl Into<String>,
        style: impl Into<String>
    ) {
        self.left_items.push(NavbarItem::new(location, text, style))
    }

    /// Add a navbar item to the left side of the navbar using the builder pattern.
    pub fn add_left_builder(
        mut self,
        location: impl Into<String>,
        text: impl Into<String>,
        style: impl Into<String>
    ) -> Self {
        self.add_left(location, text, style);
        self
    }

    /// Add a navbar item to the right side of the navbar.
    pub fn add_right(
        &mut self,
        location: impl Into<String>,
        text: impl Into<String>,
        style: impl Into<String>
    ) {
        self.right_items.push(NavbarItem::new(location, text, style))
    }


    /// Add a navbar item to the right side of the navbar using the builder pattern.
    pub fn add_right_builder(
        mut self,
        location: impl Into<String>,
        text: impl Into<String>,
        style: impl Into<String>
    ) -> Self {
        self.add_right(location, text, style);
        self
    }

    /// Render using the template registry
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("navbar", self)
    }
}