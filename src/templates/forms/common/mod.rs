//! Common components for building forms in telescope.

pub mod email;
pub mod password;

/// Common items across all form fields.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct FormFieldCommon {
    /// The name associated with this form field on submission.
    name: String,
    /// The label that appears next to this form field.
    label: String,
    /// The id of this form field in the HTML document.
    id: String,
}

impl FormFieldCommon {
    /// Construct a new set of common form field attributes.
    fn new(id: String, name: String, label: String) -> Self {
        Self { id, name, label }
    }

    /// Override the name of the parent form field.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Override the label on the parent form field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }
}
