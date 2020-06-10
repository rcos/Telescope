use handlebars::{Handlebars, RenderError};

/// An item in the top navigation bar.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NavbarItem {
    /// The location to redirect to.
    location: String,
    /// The classes that should be used to label this item.
    /// These are added to the existing w3-navbar-item class.
    classes: String,
    /// The text of the item. This may not get rendered
    /// (if using google material design font, for example, this ligatures into a single symbol)
    text: String,
}

impl NavbarItem {
    /// Create a navbar item.
    fn new(location: impl Into<String>, text: impl Into<String>, material_icon: bool) -> Self {
        Self {
            location: location.into(),
            classes: (if material_icon {"material-icons"} else {""}).to_owned(),
            text: text.into()
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
        let mut s = Self::default();
        s.add_defaults();
        s
    }

    /// Add a navbar item to the left side of the navbar.
    pub fn add_left(
        &mut self,
        location: impl Into<String>,
        text: impl Into<String>,
        material_icon: bool
    ) {
        self.left_items.push(NavbarItem::new(location, text, material_icon))
    }

    /// Add a navbar item to the left side of the navbar using the builder pattern.
    pub fn add_left_builder(
        mut self,
        location: impl Into<String>,
        text: impl Into<String>,
        material_icon: bool
    ) -> Self {
        self.add_left(location, text, material_icon);
        self
    }

    /// Add a navbar item to the right side of the navbar.
    pub fn add_right(
        &mut self,
        location: impl Into<String>,
        text: impl Into<String>,
        material_icon: bool
    ) {
        self.right_items.push(NavbarItem::new(location, text, material_icon))
    }


    /// Add a navbar item to the right side of the navbar using the builder pattern.
    pub fn add_right_builder(
        mut self,
        location: impl Into<String>,
        text: impl Into<String>,
        material_icon: bool
    ) -> Self {
        self.add_right(location, text, material_icon);
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
            self.add_left(*path, *text, false);
        }
    }

    /// Render using the template registry
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("navbar", self)
    }
}