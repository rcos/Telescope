pub mod auth;
pub mod jumbotron;
pub mod navbar;
pub mod page;
pub mod static_pages;
pub mod homepage;

use crate::app_data::AppData;
use crate::error::TelescopeError;
use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json::{Map, Value};
use std::ops::{Index, IndexMut};

/// A template that can be rendered using the handlebars template registry.
#[derive(Serialize, Debug, Clone)]
pub struct Template {
    /// The file to use to render this template.
    #[serde(skip)]
    pub handlebars_file: &'static str,

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

    /// Append fields from another object.
    /// This will panic if there is an error converting the
    /// other object into a JSON value or if the JSON value
    /// is not a JSON Object.
    pub fn append_fields(&mut self, other: impl Serialize) {
        // Convert the other object to JSON values.
        let converted: Value =
            serde_json::to_value(other).expect("Could not convert object to JSON value");

        // Get the internal JSON object.
        if let Value::Object(mut obj) = converted {
            self.fields.append(&mut obj);
        } else {
            panic!("The other object did not convert to a JSON object.");
        }
    }

    /// Builder pattern version of [Self::append_fields].
    pub fn with_fields(mut self, from: impl Serialize) -> Self {
        self.append_fields(from);
        self
    }

    /// Render this template using the global handlebars registry.
    pub fn render(&self) -> Result<String, TelescopeError> {
        AppData::global()
            .render_template(self)
            .map_err(TelescopeError::RenderingError)
    }
}

impl<T: Into<String>> Index<T> for Template {
    type Output = Value;

    fn index(&self, index: T) -> &Self::Output {
        // Immutable indexing for fields.
        &self.fields[&index.into()]
    }
}

impl<T: Into<String>> IndexMut<T> for Template {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        // Mutable indexing for fields.
        &mut self.fields[&index.into()]
    }
}

impl Responder for Template {
    type Error = TelescopeError;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let result = AppData::global()
            .render_template(&self)
            .map(|rendered: String| {
                HttpResponse::Ok()
                    .content_type("text/html;charset=UTF-8")
                    .body(rendered)
            })
            .map_err(TelescopeError::from);
        ready(result)
    }
}
