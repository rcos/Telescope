use crate::templates::Template;

/// Create a new jumbotron template.
pub fn new(heading: impl Into<String>, message: impl Into<String>) -> Template {
    let mut template = Template::new("jumbotron");

    template.fields = json!({
        "heading": heading.into(),
        "message": message.into()
    });

    return template;
}
