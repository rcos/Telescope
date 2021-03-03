//! Common form component for the submission button.

/// A submit button at the bottom of a form.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubmitButton {
    /// The text on the submit button.
    pub text: String,
    /// Any css classes to add to the submit button.
    /// When `None`, the default is `btn-primary`)
    /// All submit buttons have `btn` and `btn-spinner` already.
    pub class: Option<String>,
}
