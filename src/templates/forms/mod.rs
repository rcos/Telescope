//! Form templates, which support repeated unsuccessful submission until success
//! via a POST request and access but not submission via a GET request. Forms are
//! a special type of template commonly used in Telescope, and therefore have
//! their own traits.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Map;

pub mod common;


/// The type attribute of a text field in a form.
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum TextFieldType {
    Email,
    Password,
}

/// Form fields.
#[derive(Serialize)]
struct FormField {
    /// The path from the templates directory to the handlebars file.
    #[serde(skip)]
    handlebars_file: &'static str,

    /// The name of this field in the submitted form object.
    name: &'static str,

    /// The type attribute of this form field.
    #[serde(rename = "type")]
    ty: TextFieldType,

    /// The value to pre-fill the form field with.
    prefill: Option<String>,

    /// The place-holder to put in the form field on on value.
    placeholder: Option<String>,
}

/// A form that the user must fill out.
#[derive(Serialize)]
pub struct Form {
    /// The handlebars file that should be used to render this form.
    #[serde(skip)]
    handlebars_file: &'static str,

    /// The fields of this form.
    fields: Vec<FormField>
}
