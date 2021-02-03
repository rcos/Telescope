//! Navbar and navbar links. This file was mostly unchanged in the December 2020
//! template refactor and it is mostly fine as is at the moment.

use crate::{
    templates::{forms::login, Template},
    web::context::RequestContext,
};

use uuid::Uuid;
use crate::error::TelescopeError;
use crate::models::users::User;

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

/// Template for the navigation bar at the top of the page.
#[derive(Clone, Debug, Serialize)]
pub struct Navbar {
    left_items: Vec<NavbarLink>,
    right_items: Vec<NavbarLink>,
}

impl Navbar {
    /// The path to the template from the templates directory.
    const TEMPLATE_NAME: &'static str = "navbar";

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
            .add_left(NavbarLink::new(ctx, "/", "Home"))
            .add_left(NavbarLink::new(ctx, "/projects", "Projects"))
            .add_left(NavbarLink::new(ctx, "/developers", "Developers"))
            .add_left(NavbarLink::new(ctx, "/sponsors", "Sponsors"))
            .add_left(NavbarLink::new(ctx, "/blog", "Blog"))
    }

    /// Get a navbar without a user logged in.
    fn without_user(ctx: &RequestContext) -> Self {
        let class = "btn btn-secondary mr-2 mb-2";

        // if we are on the login page dont add another layer of redirect
        let target = if ctx.request().path() == "/login" {
            login::target_page(ctx)
        } else {
            ctx.request().uri().to_string()
        };

        let login_query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair(login::REDIRECT_QUERY_VAR, target.as_str())
            .finish();

        let login_location = format!("/login?{}", login_query);

        Self::with_defaults(ctx)
            .add_right(NavbarLink::new(ctx, login_location, "Login").class(class))
            .add_right(NavbarLink::new(ctx, "/register", "Sign Up").class(class))
    }

    /// Create a navbar based on the page context.
    pub async fn from_context(ctx: &RequestContext) -> Result<Self, TelescopeError> {
        // Get the logged in user.
        let user_login: Option<User> = ctx.user_identity().await?;

        // If there is no user return a user-less navbar.
        if user_login.is_none() {
            return Ok(Self::without_user(ctx));
        }

        // Otherwise extract the user's id.
        let user_id: Uuid = user_login.unwrap().id;

        // Convert it to the appropriate logout redirect and profile links.
        let logout_redir_query: String = url::form_urlencoded::Serializer::new(String::new())
            .append_pair(login::REDIRECT_QUERY_VAR, ctx.request().uri().path())
            .finish();

        let profile_link: String = format!("/profile/{}", user_id.to_hyphenated());
        let logout_link: String = format!("/logout?{}", logout_redir_query);

        // Put those links in navbar objects.
        let profile_btn: NavbarLink = NavbarLink::new(ctx, profile_link, "Profile")
            .class("mr-2 mb-2 btn-primary");
        let logout_btn: NavbarLink = NavbarLink::new(ctx, logout_link, "Logout")
            .class("mr-2 mb-2 btn-secondary");

        // And create a navbar object with those link objects added.
        let navbar: Navbar = Self::with_defaults(ctx)
            .add_right(profile_btn)
            .add_right(logout_btn);
        return Ok(navbar);
    }

    /// Convert this navbar to a template.
    pub fn template(&self) -> Template {
        Template::new(Self::TEMPLATE_NAME).with_fields(self)
    }
}
