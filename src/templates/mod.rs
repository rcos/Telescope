pub mod developers;
pub mod emails;
pub mod graphql_playground;
pub mod jumbotron;
pub mod navbar;
pub mod page;
pub mod profile;

pub mod forms;

/// Re-export everything in the static_pages module publicly.
pub mod static_pages;

pub use static_pages::*;

use serde_json::{Value, Map};
use serde::Serialize;
use handlebars::Handlebars;
use std::ops::{Index, IndexMut};

/// A template that can be rendered using the handlebars template registry.
#[derive(Serialize, Debug, Clone)]
pub struct Template {
    /// The file to use to render this template.
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
        self.fields[&key.into()] = serde_json::to_value(val)
            .expect("Failed to serialize value.");
    }

    /// Append fields from another object.
    /// This will panic if there is an error converting the
    /// other object into a JSON value or if the JSON value
    /// is not a JSON Object.
    pub fn append_fields(&mut self, other: impl Serialize) {
        // Convert the other object to JSON values.
        let converted: Value = serde_json::to_value(other)
            .expect("Could not convert object to JSON value");

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

    /// Render this template using a reference to the handlebars registry.
    pub fn render(&self, handlebars: &Handlebars) -> String {
        handlebars.render(self.handlebars_file, &self)
            .expect("Could not render template.")
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