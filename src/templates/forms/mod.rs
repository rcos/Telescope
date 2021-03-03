//! Form templates, which support repeated unsuccessful submission until success
//! via a POST request and access but not submission via a GET request. Forms are
//! a special type of template commonly used in Telescope, and therefore have
//! their own traits.

use serde::Serialize;
use common::text_field::TextField;
use std::collections::HashMap;
use actix_web::{Responder, HttpRequest, Error, HttpResponse, FromRequest};
use actix_web::body::Body;
use crate::error::TelescopeError;
use futures::future::{Ready, ready};
use crate::app_data::AppData;
use crate::templates::{page, Template};
use actix_web::http::header::CONTENT_TYPE;
use crate::templates::forms::common::submit_button::SubmitButton;
use serde_json::{Map, Value};
use actix_web::web::Form as ActixForm;

pub mod common;

/// A field in a form.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FormField {
    /// A text field in a form.
    TextField(TextField)
}

/// A form that the user must fill out. All forms submit by `POST` to
/// the URL they are served at.
#[derive(Serialize, Deserialize)]
pub struct Form {
    /// The path from the template root to this form's handlebars template.
    template_path: String,
    /// The page title.
    pub page_title: String,
    /// The fields of this form. Keys are `name` attributes.
    fields: HashMap<String, FormField>,
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
            fields: HashMap::new(),
            submit_button: SubmitButton {
                text: "Submit".into(),
                class: None
            },
            other: Map::new()
        }
    }

    /// Render this form.
    fn render(&self) -> Result<String, TelescopeError> {
        AppData::global()
            // Get the global handlebars registry
            .get_handlebars_registry()
            // Render the form object
            .render(self.template_path.as_str(), self)
            // Convert and propagate any errors.
            .map_err(TelescopeError::RenderingError)
    }

    /// Try to validate this form using the input in the HTTP request. Return an error if the request
    /// is malformed or if the form fails to validate.
    async fn validate_input(req: &HttpRequest) -> Result<HashMap<String, String>, TelescopeError> {
        // Deserialize the form fields from the request into a hash map.
        let form_input: HashMap<String, String> = ActixForm::<HashMap<String, String>>::extract(req)
            .await
            // Convert and propagate errors as necessary.
            .map_err(|e| TelescopeError::bad_request(
                "Malformed Form Data",
                format!("Could not deserialize form data. Internal error: {}", e)))?.0;

        unimplemented!()
    }
}

impl Responder for Form {
    type Error = TelescopeError;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        // Render this form
        ready(self.render()
            // Put this form in a page.
            .and_then(|rendered| page::with_content(
                // With the request path
                req.path(),
                // The page title
                self.page_title,
                // And the rendered form
                rendered.as_str()))
            // Render the page to HTML
            .and_then(|page| page.render())
            // Use the rendered page as the body of the response
            .map(|rendered| HttpResponse::Ok()
                .header(CONTENT_TYPE, "text/html;charset=UTF-8")
                .body(rendered)))
    }
}