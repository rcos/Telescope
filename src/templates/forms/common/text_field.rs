//! Short text field in a form.

use crate::templates::forms::{Form, FormField};

/// Path to the
const TEMPLATE_PATH: &'static str = "forms/common/text_field";

/// Form fields.
#[derive(Serialize, Deserialize)]
pub struct TextField {
    /// The name of this field in the submitted form object.
    name: String,

    /// The value to pre-fill the form field with.
    value: Option<String>,

    /// If there was an error with this form field, display this error message.
    error: Option<String>,

    /// If this form field did not error, display this success message.
    success: Option<String>,

    /// Function to validate an input and return this field. This should set the
    /// `is_valid` field of this object.
    #[serde(skip)]
    validator: Option<Box<dyn Fn(Option<String>) -> Self>>,

    /// Flag for the validator to set to indicate if a form is valid.
    #[serde(skip)]
    is_valid: Option<bool>,
}

impl TextField {
    /// Validate this field for a given input. Panic if there is no validator
    /// function.
    pub fn validate(self, value: Option<String>) -> Self {
        // Get the validation function
        let validator = self.validator
            .expect("Text field validator missing");
        // Call and return the validation function.
        return validator(value);
    }

    /// Check the output of the validator function. This
    pub fn validator_errored(&self) -> bool {
        self.is_valid.expect("Validator has not set validity field")
    }
}

impl Form {
    /// Add a text field to a form. Panic on trying to overwrite an existing field.
    fn add_text_field(&mut self, text_field: TextField) -> &mut Form {
        if self.fields.contains_key(text_field.name.as_str()) {
            panic!("Cannot overwrite existing field in form");
        } else {
            self.fields.insert(text_field.name.clone(), FormField::TextField(text_field));
            self
        }
    }
}
