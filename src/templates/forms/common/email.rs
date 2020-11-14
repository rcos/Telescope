use crate::templates::forms::common::FormFieldCommon;
use crate::web::Template;

/// Email field in a form. Supports a prefill and error message.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailField {
    /// Common fields of this form.
    #[serde(flatten)]
    common: FormFieldCommon,
    /// Email prefill. If present, placed in the form instead of the placeholder.
    prefill: Option<String>,
    /// An optional error message to appear in a tooltip.
    error: Option<String>,
}

impl EmailField {
    /// Create a new email field identified by a document id.
    /// Label defaults to "Email Address".
    /// Name defaults to "email".
    pub fn new(id: impl Into<String>) -> Self {
        let common = FormFieldCommon::new(id.into(), "email".into(), "Email Address".into());
        Self {
            prefill: None,
            error: None,
            common,
        }
    }

    /// Builder method to set the prefill on an email field.
    pub fn prefill(mut self, prefill: impl Into<String>) -> Self {
        self.prefill = Some(prefill.into());
        self
    }

    /// Builder method to set the error message on an email field.
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    /// Apply a set of operations to the common fields of this form.
    pub fn map_common<F: FnOnce(FormFieldCommon) -> FormFieldCommon>(mut self, f: F) -> Self {
        self.common = f(self.common);
        self
    }
}

impl Template for EmailField {
    const TEMPLATE_NAME: &'static str = "forms/common/email";
}
