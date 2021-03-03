//! Form templates, which support repeated unsuccessful submission until success
//! via a POST request and access but not submission via a GET request. Forms are
//! a special type of template commonly used in Telescope, and therefore have
//! their own traits.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Map;
use common::text_field::TextField;
use std::collections::HashMap;

mod common;

/// Path from the template root to the form template file.
const TEMPLATE_PATH: &'static str = "forms/form";

/// A field in a form.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum FormField {
    /// A text field in a form.
    TextField(TextField)
}

/// A form that the user must fill out. All forms submit by `POST` to
/// the URL they are served at.
#[derive(Serialize)]
pub struct Form {
    /// The title that will appear above the form.
    pub form_title: String,
    /// The fields of this form. Keys are `name` attributes.
    fields: HashMap<String, FormField>,
    /// The text on the submit button.
    pub submit_button_text: String,
    /// Any css classes to add to the submit button.
    /// When `None`, the default is `btn-primary`)
    /// All submit buttons have `btn` and `btn-spinner` already.
    pub submit_button_class: Option<String>,
}

impl Form {
    /// Create a new empty form.
    fn empty(title: impl Into<String>) -> Self {
        Self {
            form_title: title.into(),
            fields: HashMap::new(),
            submit_button_text: "Submit".into(),
            submit_button_class: None
        }
    }
}
