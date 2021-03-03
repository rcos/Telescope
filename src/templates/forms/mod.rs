//! Form templates, which support repeated unsuccessful submission until success
//! via a POST request and access but not submission via a GET request. Forms are
//! a special type of template commonly used in Telescope, and therefore have
//! their own traits.

use serde::Serialize;
use common::text_field::TextField;
use std::collections::HashMap;
use actix_web::{Responder, HttpRequest, Error, HttpResponse};
use actix_web::body::Body;
use crate::error::TelescopeError;
use futures::future::{Ready, ready};
use crate::app_data::AppData;
use crate::templates::{page, Template};
use actix_web::http::header::CONTENT_TYPE;

pub mod common;

/// Path from the template root to the form template file.
const TEMPLATE_PATH: &'static str = "forms/form";

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
    /// The page title.
    pub page_title: String,
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
        let title = title.into();
        Self {
            page_title: title.clone(),
            form_title: title,
            fields: HashMap::new(),
            submit_button_text: "Submit".into(),
            submit_button_class: None
        }
    }

    /// Render this form.
    fn render(&self) -> Result<String, TelescopeError> {
        AppData::global()
            // Get the global handlebars registry
            .get_handlebars_registry()
            // Render the form object
            .render(TEMPLATE_PATH, self)
            // Convert and propagate any errors.
            .map_err(TelescopeError::RenderingError)
    }

    /// Try to validate this form. Return an error if the request is malformed
    /// or if the form fails to validate.
    fn validate_input() -> Result<HashMap<String, String>, TelescopeError> {
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