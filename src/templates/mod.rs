use std::ops::{Index, IndexMut};

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::app_data::AppData;
use crate::error::TelescopeError;

pub mod auth;
pub mod forms;
pub mod helpers;
pub mod jumbotron;
pub mod navbar;
pub mod page;
pub mod pagination;
pub mod static_pages;

/// A template that can be rendered using the handlebars template registry.
#[derive(Serialize, Debug, Clone)]
pub struct Template {
    /// The file to use to render this template.
    #[serde(skip)]
    handlebars_file: &'static str,

    /// The fields to render.
    #[serde(flatten)]
    fields: Map<String, Value>,
}

impl Template {
    /// Create a new template object with the path to the handlebars file from
    /// the templates directory.
    pub fn new(path: &'static str) -> Self {
        Self {
            handlebars_file: path,
            fields: Map::new(),
        }
    }

    /// Builder style method to add a field to this template instance.
    /// This will panic if there is a serialization failure.
    pub fn field(mut self, key: impl Into<String>, val: impl Serialize) -> Self {
        self.set_field(key, val);
        self
    }

    /// Setter method for fields on this template instance.
    /// This will panic if there is a serialization failure.
    pub fn set_field(&mut self, key: impl Into<String>, val: impl Serialize) {
        let serialized_val = serde_json::to_value(val).expect("Failed to serialize value");
        self.fields.insert(key.into(), serialized_val);
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

    /// Render this template as the content of a page.
    pub async fn render_into_page(
        &self,
        req: &HttpRequest,
        title: impl Into<Value>,
    ) -> Result<Template, TelescopeError> {
        page::of(req, title, self).await
    }
}

impl<T: Into<String>> Index<T> for Template {
    type Output = Value;

    /// Returns [`Value::Null`] if the key is not in the template.
    fn index(&self, index: T) -> &Self::Output {
        // Immutable indexing for fields.
        self.fields
            .get(index.into().as_str())
            .unwrap_or(&Value::Null)
    }
}

impl<T: Into<String>> IndexMut<T> for Template {

    /// Returns the existing value or creates a new empty object at the location
    /// and returns a reference to that.
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.fields
            // Get the existing entry if available
            .entry(index)
            // Or insert an empty object.
            .or_insert(json!({}))
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
