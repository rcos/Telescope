//! Short text field in a form.

/// Path to the
const TEMPLATE_PATH: &'static str = "forms/common/text_field";

/// The type attribute of a text field in a form.
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TextFieldType {
    Email,
    Password,
}

/// Form fields.
#[derive(Serialize)]
pub struct TextField {
    /// The HTML element id of this form item.
    id: String,

    /// The label that appears next to this text field.
    label: String,

    /// The name of this field in the submitted form object.
    name: &'static str,

    /// The type attribute of this form field.
    #[serde(rename = "type")]
    ty: TextFieldType,

    /// The value to pre-fill the form field with.
    prefill: Option<String>,

    /// The place-holder to put in the form field on on value.
    placeholder: Option<String>,

    /// If there was an error with this form field, display this error message.
    error: Option<String>,

    /// If this form field did not error, display this success message.
    success: Option<String>,

    /// Function to validate an input and return this field.
    #[serde(skip)]
    validator: Box<dyn Fn(Option<String>) -> Self>,

    /// Flag for the validator to set to indicate if a form is valid.
    #[serde(skip)]
    is_valid: bool,
}
