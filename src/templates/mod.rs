use std::ops::{Index, IndexMut};

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::app_data::AppData;
use crate::error::TelescopeError;
use crate::templates::page::Page;

pub mod auth;
pub mod helpers;
pub mod jumbotron;
pub mod navbar;
pub mod page;
pub mod pagination;
pub mod static_pages;
pub mod tags;

/// A template that can be rendered using the handlebars template registry.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    /// The file to use to render this template.
    pub handlebars_file: String,

    /// The fields to render.
    pub fields: Value,
}

impl Template {
    /// Create a new template object with the path to the handlebars file from
    /// the templates directory.
    pub fn new(path: &'static str) -> Self {
        Self {
            handlebars_file: path.into(),
            fields: json!({}),
        }
    }

    /// Render this template using the global handlebars registry.
    pub fn render(&self) -> Result<String, TelescopeError> {
        AppData::global()
            // Get the global handlebars registry
            .get_handlebars_registry()
            // Render this template's file with this template's data
            .render(self.handlebars_file.as_str(), &self.fields)
            // Convert any rendering errors that occur.
            .map_err(TelescopeError::RenderingError)
    }

    /// Render this template as the content of a page.
    pub async fn in_page(self, req: &HttpRequest, title: impl Into<String>) -> Result<Page, TelescopeError> {
        Page::new(req, title, self).await
    }
}

impl<T> Index<T> for Template
where T: serde_json::value::Index {
    type Output = Value;

    /// Returns [`Value::Null`] if the key is not in the template.
    fn index(&self, index: T) -> &Self::Output {
        // Immutable indexing for fields.
        self.fields.index(index)
    }
}

impl<T> IndexMut<T> for Template
where T: serde_json::value::Index {
    /// Returns the existing value or creates a new empty object at the location
    /// and returns a reference to that.
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.fields.index_mut(index)
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
