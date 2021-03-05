//! Form templates, which support repeated unsuccessful submission until success
//! via a POST request and access but not submission via a GET request. Forms are
//! a special type of template commonly used in Telescope, and therefore have
//! their own traits.

use serde::Serialize;
use common::text_field::TextField;
use std::collections::HashMap;
use actix_web::{Responder, HttpRequest, Error, HttpResponse, FromRequest, HttpMessage};
use actix_web::body::Body;
use crate::error::TelescopeError;
use futures::future::{Ready, ready, LocalBoxFuture};
use crate::app_data::AppData;
use crate::templates::{page, Template};
use actix_web::http::header::CONTENT_TYPE;
use crate::templates::forms::common::submit_button::SubmitButton;
use serde_json::{Map, Value};
use actix_web::web::Form as ActixForm;

pub mod common;
pub mod register;

/// A field in a form.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FormField {
    /// A text field in a form.
    TextField(TextField)
}

impl FormField {
    /// Validate a form field for a given input. Update the value of this field
    /// with any validation issues. Return if the field is valid for the input.
    fn validate(self, input: Option<&String>) -> Self {
        match self {
            // Text fields cary their own validator
            FormField::TextField(text_field) =>
                FormField::TextField(text_field.validate(input))
        }
    }

    /// If this field of the form valid?
    fn is_valid(&self) -> bool {
        match self {
            // Text fields have a validity field
            FormField::TextField(text_field) => text_field.is_valid
                // The validity field should be set by all validators
                .expect("This form field has not been validated.")
        }
    }
}

/// Form input type that can be extracted from HTTP requests and is passed to
/// the telescope form validation system.
pub type FormInput = ActixForm<HashMap<String, String>>;

/// A form that the user must fill out. All forms submit by `POST` to
/// the URL they are served at.
#[derive(Serialize, Deserialize)]
pub struct Form {
    /// The path from the template root to this form's handlebars template.
    template_path: String,
    /// The page title.
    pub page_title: String,
    /// The fields of this form. Keys are `name` attributes.
    form_fields: HashMap<String, FormField>,
    /// The button component at the bottom of this form to submit it.
    pub submit_button: SubmitButton,
    /// Any other handlebars fields needed to render this form. These should
    /// not be form fields.
    #[serde(flatten)]
    other: Map<String, Value>
}

impl Form {
    /// Create a new empty form.
    pub fn new(path: impl Into<String>, page_title: impl Into<String>) -> Self {
        Self {
            template_path: path.into(),
            page_title: page_title.into(),
            form_fields: HashMap::new(),
            submit_button: SubmitButton {
                text: "Submit".into(),
                class: None
            },
            other: Map::new()
        }
    }

    /// Render this form.
    pub fn render(&self) -> Result<String, TelescopeError> {
        AppData::global()
            // Get the global handlebars registry
            .get_handlebars_registry()
            // Render the form object
            .render(self.template_path.as_str(), self)
            // Convert and propagate any errors.
            .map_err(TelescopeError::RenderingError)
    }

    /// Add a non-form field key to the form.
    pub fn add_other_key(&mut self, key: impl Into<String>, value: impl Serialize) -> &mut Self {
        // Serialize the value.
        let serialized = serde_json::to_value(value)
            .expect("Serialization error");

        // Add it to this form/template
        self.other.insert(key.into(), serialized);

        return self;
    }

    /// Builder style function to add a non-form-field key to a template.
    pub fn with_other_key(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.add_other_key(key, value);
        self
    }

    /// Try to validate this form using the form input extracted from the request. Return an error
    /// if the form fails to validate.
    pub async fn validate_input(&mut self, form_input: ActixForm<HashMap<String, String>>) -> Result<HashMap<String, String>, TelescopeError> {
        // Create map to put validated fields in
        let mut validated_fields: HashMap<String, FormField> = HashMap::with_capacity(self.form_fields.len());
        let mut form_valid: bool = true;

        // For each field in this form, validate that field against the submitted input.
        for (name, field) in self.form_fields.drain() {
            // Get field input
            let input: Option<&String> = form_input.get(name.as_str());

            // Add the validated field to the new form field map. Update form
            // validity flag.
            let validated: FormField = field.validate(input);
            form_valid &= validated.is_valid();
            validated_fields.insert(name, validated);
        }

        // Replace this forms fields with the validated ones.
        self.form_fields = validated_fields;
        // If the form was invalid, return it to the user in an error.
        if !form_valid {
            Err(TelescopeError::invalid_form(self))
        } else {
            // Otherwise return the validated field values.
            return Ok(form_input.0);
        }
    }
}

impl Responder for Form {
    type Error = TelescopeError;
    type Future = LocalBoxFuture<'static, Result<HttpResponse, Self::Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        // Clone the request to satisfy lifetime constraints. This won't cause
        // issues, since the request is a wrapper around a shared pointer.
        let req = req.clone();

        return Box::pin(async move {
            // Render this form.
            let rendered: String = self.render()?;

            // Put it in a page.
            page::with_content(&req, self.page_title, rendered.as_str())
                // Wait for the page to resolve the username etc
                .await?
                // Render the page to HTML
                .render()
                // Use the rendered page as the body of the response
                .map(|rendered| HttpResponse::Ok()
                    .header(CONTENT_TYPE, "text/html;charset=UTF-8")
                    .body(rendered))
        });
    }
}
