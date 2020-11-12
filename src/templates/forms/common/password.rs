use crate::web::Template;
use crate::templates::forms::common::FormFieldCommon;

/// A password field in an html form.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordField {
    /// The common items of this field.
    #[serde(flatten)]
    common: FormFieldCommon,
    /// An optional error message if this field is invalid.
    error: Option<String>
}

impl PasswordField {
    /// Construct a new password field with the given id.
    /// Name defaults to "password".
    pub fn new(id: impl Into<String>) -> Self {
        let common = FormFieldCommon::new(
            id.into(),
            "password".to_string(),
            "Password".to_string()
        );
        Self {
            common,
            error: None
        }
    }

    /// Apply a set of operations to the common fields of this form.
    pub fn map_common<F: FnOnce(FormFieldCommon) -> FormFieldCommon>(mut self, f: F) -> Self {
        self.common = f(self.common);
        self
    }

    /// Set an error message on this field.
    pub fn error(mut self, message: impl Into<String>) -> Self {
        self.error = Some(message.into());
        self
    }
}

impl Template for PasswordField {
    const TEMPLATE_NAME: &'static str = "forms/common/password";
}
