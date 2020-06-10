use handlebars::{Handlebars, RenderError};

/// The Container template is just a wrapper around a container div.
/// This is just used to align the site.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Container {
    /// The html rendered in this container div.
    inner: String
}

impl Container {
    /// Create a new container.
    pub fn new(str: impl Into<String>) -> Self {
        Self {
            inner: str.into()
        }
    }

    /// Render this container using a the handlebars registry.
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("container", self)
    }

    /// Wrap a string in a container div and render using the registry.
    pub fn wrap(&self, registry: &Handlebars, inner: impl Into<String>) -> Result<String, RenderError> {
        registry.render("container", &Self::new(inner))
    }
}