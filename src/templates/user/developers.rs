//! Developers page template fields and functions

use crate::templates::Template;
use crate::web::api::rcos::users::developers_page::DevelopersResponse;
use crate::web::services::user::developers::DevelopersPageQuery;

/// The path to the developers page template from the templates directory.
const TEMPLATE_PATH: &'static str = "user/developers";

/// The handlebars key for the list of users to display.
pub const USERS: &'static str = "users";

/// The handlebars key for the query parameters.
pub const QUERY: &'static str = "query";
/// The handlebars key for a user's username.
pub const USERNAME: &'static str = "username";

/// The handlebars key for a user's first name.
pub const FNAME: &'static str = "first_name";

/// The handlebars key for a user's last name.
pub const LNAME: &'static str = "last_name";

/// Create the developers page template
pub async fn developers(
    query: &DevelopersPageQuery,
    response_data: &DevelopersResponse,
) -> Template {
    Template::new(TEMPLATE_PATH).field(QUERY, query)
}
