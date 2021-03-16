//! Developers page template fields and functions

use crate::web::services::developers::DevelopersPageQuery;
use crate::web::api::rcos::users::developers_page::DevelopersResponse;
use crate::templates::Template;
use crate::web::telescope_ua;
use crate::env::global_config;
use crate::error::TelescopeError;

/// The path to the developers page template from the templates directory.
const TEMPLATE_PATH: &'static str = "developers";

/// The handlebars key for the list of users to display.
pub const USERS: &'static str = "users";

/// The handlebars key for the query parameters.
pub const QUERY: &'static str = "query";

/// The handlebars key for a user's profile picture.
pub const PROFILE_PICTURE: &'static str = "pfp";

/// The handlebars key for a user's username.
pub const USERNAME: &'static str = "username";

/// The handlebars key for a user's first name.
pub const FNAME: &'static str = "first_name";

/// The handlebars key for a user's last name.
pub const LNAME: &'static str = "last_name";

/// Default URL for user's profile pics when there is not an existing one for that user. We use the
/// Gravatar mystery-person.
pub const DEFAULT_PFP_URL: &'static str = "https://www.gravatar.com/avatar/00000000000000000000000000000000?d=mp&f=y&r=g";

/// Get a user's github profile picture.
async fn get_pfp_url_github(github_user_id: String) -> Result<String, TelescopeError> {
    // Get the github credentials from the global config.
    let github_client_id = global_config().github_credentials.client_id.to_string();
    let github_client_secret = global_config().github_credentials.client_secret.secret().to_string();

    // Make GitHub client using hubcaps.
    let github_client = Github::new(
        telescope_ua(),
        Credentials::Client(github_client_id, github_client_secret))?;

    // Use the GitHub client to get the user's profile picture.
    // FIXME: Figure out how to get github users by ID or find other workaround.
    // github_client.users().get()
    unimplemented!()
}

/// Create the developers page template
pub async fn developers(query: &DevelopersPageQuery, response_data: &DevelopersResponse) -> Template {
    Template::new(TEMPLATE_PATH)
        .field(QUERY, query)
}
