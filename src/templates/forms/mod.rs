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

pub mod meeting;
pub mod register;

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
