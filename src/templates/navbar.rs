use crate::{
    web::{RequestContext, Template},
    templates::login::LoginForm,
};

use uuid::Uuid;

/// A button that just links to a another part of the site (or another site entirely.)
/// This is good for most items on the header bar.
#[derive(Clone, Debug, Serialize)]
pub struct NavbarLink {
    /// This is the active (current) page.
    is_active: bool,
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
    pub fn new(ctx: &RequestContext, location: impl Into<String>, text: impl Into<String>) -> Self {
        let loc = location.into();
        Self {
            is_active: ctx.request().path() == &loc,
            location: loc.clone(),
            text: text.into(),
            class: "nav-link".to_string(),
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

/// A navbar definition.
#[derive(Clone, Debug, Serialize)]
pub struct Navbar {
    left_items: Vec<NavbarLink>,
    right_items: Vec<NavbarLink>,
}

impl Navbar {
    /// Get an empty navbar object.
    const fn empty() -> Self {
        Self {
            left_items: Vec::new(),
            right_items: Vec::new(),
        }
    }

    fn add_left(mut self, item: NavbarLink) -> Self {
        self.left_items.push(item);
        self
    }

    fn add_right(mut self, item: NavbarLink) -> Self {
        self.right_items.push(item);
        self
    }

    /// Navbar with homepage, achievement page, projects, developers and sponsors
    fn with_defaults(ctx: &RequestContext) -> Self {
        Self::empty()
            .add_left(NavbarLink::new(ctx,"/", "Home"))
            .add_left(NavbarLink::new(ctx,"/projects", "Projects"))
            .add_left(NavbarLink::new(ctx,"/developers", "Developers"))
            .add_left(NavbarLink::new(ctx,"/sponsors", "Sponsors"))
            .add_left(NavbarLink::new(ctx,"/blog", "Blog"))
    }

    /// Get a navbar without a user logged in.
    fn without_user(ctx: &RequestContext) -> Self {
        let class = "btn btn-secondary mr-2 mb-2";
        // use url encoder to do any necessary escaping.
        let login_query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair(
                LoginForm::REDIRECT_QUERY_VAR,
                ctx.request().uri().to_string().as_str())
            .finish();
        let login_location= format!("/login?{}", login_query);
        Self::with_defaults(ctx)
            .add_right(NavbarLink::new(ctx, login_location, "Login",).class(class))
            .add_right(NavbarLink::new(ctx,"/register", "Sign Up").class(class))
    }

    /// Create a navbar based on the page context.
    pub fn from_context(ctx: &RequestContext) -> Self {
        ctx.identity()
            .identity()
            .and_then(|id: String| Uuid::parse_str(id.as_str()).ok())
            .map_or(Self::without_user(ctx), |uuid| Self::with_defaults(ctx)
                .add_right(NavbarLink::new(ctx,
                        format!("/profile/{}", uuid.to_hyphenated()),
                        "Profile"
                    ).class("mr-2 mb-2 btn btn-primary"),
                )
                .add_right(NavbarLink::new(ctx,"/logout", "Logout")
                               .class("mr-2 mb-2 btn btn-secondary"),
                )
                // Add API access for users.
                .add_left(NavbarLink::new(ctx,"/playground", "API Playground"))
            )
    }
}

impl Template for Navbar {
    const TEMPLATE_NAME: &'static str = "navbar";
}
