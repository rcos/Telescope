use crate::templates::Template;

/// The path from the templates directory to the handlebars file.
const TEMPLATE_FILE: &'static str = "forms/common/text_field";

/// The HTML id of the form field.
pub const ID_FIELD: &'static str = "id";

/// The name of the form field. This corresponds to the name of the value
/// in the url encoded data received when the form is submitted.
pub const NAME_FIELD: &'static str = "name";

/// The text that appears next to the field.
pub const LABEL_FIELD: &'static str = "label";

/// The value that is prefilled into the field.
pub const PREFILL_FIELD: &'static str = "prefill";

/// The value that appears in the field when there is no existing data.
pub const PLACEHOLDER_FIELD: &'static str = "placeholder";

/// The field that indicates the type of the text field. This should be used
/// with the text field enum here.
pub const TYPE_FIELD: &'static str = "type";

/// The type of text that is acceptable into a text field. One of these
/// should be the value of the `type` field for every text field.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TextFieldType {
    Password,
    Email,
    Text,
}

/// The field for error messages shown with a form field. This is optional
/// and if present, the text field will be marked invalid.
pub const ERROR_FIELD: &'static str = "error";

/// The field for messages to display on a successful operation.
/// Functions similarly to (and conflicts with) the error field.
pub const SUCCESS_FIELD: &'static str = "success";

/// Create a new text field template for a form.
///
/// ## Arguments:
/// - `field_name`: The name of the field in the form. This matches the
///     name in the url encoded data received when the form is submitted.
///     The HTML element id is also set to equal this value but can be
///     overridden via the `id` field.
///
/// - `ty`: The HTML input type of the form field. See
///     https://www.w3schools.com/html/html_form_input_types.asp for
///     details/examples.
///
/// - `label`: The text that appears next to this form field.
pub fn new(field_name: &str, ty: TextFieldType, label: &str) -> Template {
    Template::new(TEMPLATE_FILE)
        .field(NAME_FIELD, field_name)
        .field(TYPE_FIELD, ty)
        .field(ID_FIELD, field_name)
        .field(LABEL_FIELD, label)
}

/// Make an email field. See [`new`]. Adds placeholder.
pub fn email_field(name: &str, label: &str) -> Template {
    new(name, TextFieldType::Email, label)
        .field(PLACEHOLDER_FIELD, "example@email.com")
}

/// Make a password field. See [`new`].
pub fn password_field(name: &str, label: &str) -> Template {
    new(name, TextFieldType::Password, label)
}

/// Make a plain text field. See [`new`].
pub fn plaintext_field(name: &str, label: &str) -> Template {
    new(name, TextFieldType::Text, label)
}
