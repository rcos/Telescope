//! Registration form and constants.

use crate::api::rcos::users::UserAccountType;
use crate::error::TelescopeError;
use crate::templates::forms::FormTemplate;
use crate::web::services::auth::identity::RootIdentity;
use crate::web::services::auth::rpi_cas::RpiCasIdentity;

/*
/// Create a first or last name field that validates on all non-empty strings.
fn make_name_field(name: impl Into<String>) -> TextField {
    // Convert the name string to an owned value so that it can be passed to
    // the closure constructor.
    let name_str: String = name.into();

    TextField::new(name_str.clone(), |input: Option<&String>| -> TextField {
        // Create the resultant text field (with this same validator function).
        let mut result: TextField = make_name_field(name_str);

        // First/last name has to exits, and be longer than zero bytes.
        if let Some(name_str) = input {
            // Trim unicode whitespace off of the name
            let name_str = name_str.trim();
            // If it's not empty it's valid.
            if !name_str.is_empty() {
                // The name field is not empty, and is therefore valid!
                result.value = Some(name_str.to_string());
                result.success = Some("Looks Good!".into());
                result.is_valid = Some(true);
                return result;
            }
        }
        // On no/empty name, return invalid.
        result.error = Some("Cannot be empty".into());
        result.is_valid = Some(false);
        result
    })
}
*/