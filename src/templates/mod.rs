use std::ops::{Index, IndexMut};

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json::Value;

use crate::app_data::AppData;
use crate::error::TelescopeError;

pub mod auth;
pub mod forms;
pub mod helpers;
pub mod jumbotron;
pub mod page;
pub mod pagination;
pub mod static_pages;

/// A template that can be rendered using the handlebars template registry.
#[derive(Serialize, Debug, Clone)]
pub struct Template {
    /// The file to use to render this template.
    #[serde(skip)]
    pub handlebars_file: &'static str,

    /// The fields to render.
    #[serde(flatten)]
    pub fields: Value,
}

impl Template {
    /// Create a new template object with the path to the handlebars file from
    /// the templates directory.
    pub fn new(path: &'static str) -> Self {
        Self {
            handlebars_file: path,
            fields: Value::Null,
        }
    }

    /// Render this template using the global handlebars registry.
    pub fn render(&self) -> Result<String, TelescopeError> {
        AppData::global()
            // Get the global handlebars registry
            .get_handlebars_registry()
            // Render this template's file with this template's data
            .render(self.handlebars_file, self)
            // Convert any rendering errors that occur.
            .map_err(TelescopeError::RenderingError)
    }
}

impl Responder for Template {
    type Error = TelescopeError;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let result = self
            // Render this template
            .render()
            // Convert the rendered string into am HTML type response.
            .map(|rendered: String| {
                HttpResponse::Ok()
                    .content_type("text/html;charset=UTF-8")
                    .body(rendered)
            });

        // return immediately ready future
        return ready(result);
    }
}
