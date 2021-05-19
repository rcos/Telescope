//! Form templates, which support repeated unsuccessful submission until success
//! via a POST request and access but not submission via a GET request. Forms are
//! a special type of template commonly used in Telescope, and therefore have
//! their own traits.

use crate::app_data::AppData;
use crate::error::TelescopeError;
use crate::templates::page;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::web::Form as ActixForm;
use actix_web::{HttpRequest, HttpResponse, Responder};
use common::text_field::TextField;
use futures::future::LocalBoxFuture;
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashMap;
use serde::de::DeserializeOwned;

pub mod common;
pub mod meeting;
pub mod register;

/// Form input type that can be extracted from HTTP requests and is passed to
/// the telescope form validation system.
pub type FormInput = ActixForm<HashMap<String, String>>;

/// Form trait to be implemented by all forms. This tells each individual form
/// how to validate. Most implementors should be ZSTs since this trait
/// does not have any functions that reference self.
#[async_trait]
pub trait Form {
    /// The input to the form. This should be a struct or a hash map usually.
    type Input: Serialize + DeserializeOwned;

    /// The function to validate the input to this form. If the validation is
    /// successful then the input is considered safe and passed on. If there
    /// are issues with the input then an error should be returned. This error
    /// can contain an invalid form to send back to the user for them to fix,
    /// or can be any other telescope error variant.
    async fn validate(input: Self::Input) -> Result<Self::Input, TelescopeError>;

    /// Wrapper function used to convert Actix-web's form input object to this
    /// form's input and pass it to the validation function.
    async fn wrap(input: FormInput) -> Result<Self::Input, TelescopeError> {
        // Convert input to json object
        let json_value = serde_json::to_value(input.0)
            // This should not fail.
            .expect("Form input could not be serialized to a JSON object.");

        // Convert json object to form input.
        let form_input = serde_json::from_value::<Self::Input>(json_value)
            .map_err(|err| {
                error!("Form input could not be converted from JSON to rust object: {}", err);
                TelescopeError::BadRequest {
                    header: "Form Input Invalid".to_string(),
                    message: format!("The input posted to this form does not match the form's \
                    defined structure. Please contact a Coordinator and file a GitHub issue. \
                    Internal error: {}", err),
                    show_status_code: false
                }
            })?;

        // Pass converted data to the validation function
        return Self::validate(form_input).await;
    }
}

/// A form that the user must fill out. All forms submit by `POST` to
/// the URL they are served at.
#[derive(Serialize, Deserialize, Debug)]
pub struct FormTemplate {
    /// The path from the template root to this form's handlebars template.
    template_path: String,

    /// The page title.
    pub page_title: String,

    /// Handlebars JSON value to render this form.
    template: Value,
}

impl FormTemplate {
    /// Create a new empty form.
    pub fn new(template_path: impl Into<String>, page_title: impl Into<String>) -> Self {
        Self {
            template_path: template_path.into(),
            page_title: page_title.into(),
            template: Value::Null,
        }
    }

    /// Render this form.
    pub fn render(&self) -> Result<String, TelescopeError> {
        AppData::global()
            // Get the global handlebars registry
            .get_handlebars_registry()
            // Render the form object
            .render(self.template_path.as_str(), &self.template)
            // Convert and propagate any errors.
            .map_err(TelescopeError::RenderingError)
    }
}


impl Responder for FormTemplate {
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
                .map(|rendered| {
                    HttpResponse::Ok()
                        .header(CONTENT_TYPE, "text/html;charset=UTF-8")
                        .body(rendered)
                })
        });
    }
}
