//! Navbar template constants and functions.

use crate::api::rcos::users::navbar_auth::Authentication;
use crate::error::TelescopeError;
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};
use actix_web::FromRequest;
use actix_web::HttpRequest;
use uuid::Uuid;

/// The values used for rendering the navbar template at the top of every page.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Navbar {
    /// If the currently signed in user is an admin.
    is_admin: bool,
    /// If the currently signed in user is a coordinator.
    is_coordinator: bool,
    /// If the currently signed in user is a mentor.
    is_mentor: bool,
    /// If the currently signed in user is a student.
    is_student: bool,
    /// The user ID of the currently signed in user.
    user_id: Option<Uuid>,
    /// If the viewer is creating an account.
    creating_account: bool,
    /// The path of the request to mark a navbar item as active or not.
    req_path: String
}

impl Navbar {
    /// Create the most empty navbar with all default values.
    fn empty() -> Self {
        Navbar {
            is_admin: false,
            is_coordinator: false,
            is_mentor: false,
            is_student: false,
            user_id: None,
            creating_account: false,
            req_path: "".to_string()
        }
    }

    /// Create a navbar for a viewer without an account. This is the default navbar.
    fn userless(request: &HttpRequest) -> Self {
        Navbar {
            req_path: request.path().to_string(),
            // Fill remaining fields from empty navbar.
            ..Self::empty()
        }
    }

    /// Create a navbar and fill appropriately based on request parameters.
    pub async fn for_request(request: &HttpRequest) -> Result<Self, TelescopeError> {
        // Extract the authenticated identities from the request.
        let identity: Option<AuthenticationCookie> = Identity::extract(request)
            .await?
            .identity()
            .await;

        // If the user is authenticated.
        if let Some(authenticated) = identity {
            // Create a navbar instance to modify and return.
            let mut navbar = Self::userless(request);

            // Check if there is an authenticated RCOS account
            if let Some(user_id) = authenticated.get_user_id().await? {
                // If there is make a navbar with the user ID.
                // Get the navbar auth for this user.
                let navbar_auth = Authentication::get(user_id).await?;
                // Modify navbar as necessary.
                navbar.user_id = Some(user_id);
                navbar.is_admin = navbar_auth.is_admin();
                navbar.is_coordinator = navbar_auth.is_coordinating();
                navbar.is_mentor = navbar_auth.is_mentoring();
                navbar.is_student = navbar_auth.is_student();
                // Return modified navbar.
                return Ok(navbar);
            } else {
                // Otherwise the user is in the middle of creating an account.
                navbar.creating_account = true;
                return Ok(navbar);
            }
        } else {
            // If the user is not authenticated or there is no user ID, return a default navbar.
            return Ok(Self::userless(request));
        }
    }
}
