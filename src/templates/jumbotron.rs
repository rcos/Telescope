//! Jumbotron template.

use crate::templates::Template;

/// Create a new jumbotron template.
pub fn new(heading: impl Into<String>, message: impl Into<String>) -> Template {
    Template {
        handlebars_file: "jumbotron",
        fields: json!({
            "heading": heading.into(),
            "message": message.into()
        })
    }
}
