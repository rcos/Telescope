//! API parameters to select columns in output.

/// A query parameter to indicate which columns to select from a table.
#[derive(Clone, Debug, Serialize)]
pub struct SelectParameter {
    select: String
}

impl Default for SelectParameter {
    fn default() -> Self {
        // By default, select all columns.
        Self {
            select: "*".into()
        }
    }
}

impl SelectParameter {
    /// Select all columns from a table. This is the default behavior.
    pub fn all() -> Self {
        SelectParameter::default()
    }

    /// Construct a select parameter from a list of select items.
    pub fn from_items(items: Vec<SelectItem>) -> Self {
        Self {
            // Iterate over the selected columns.
            select: items.into_iter()
                // Convert each into a select parameter item string
                .map(|item| item.into())
                // Collect into a vector
                .collect::<Vec<String>>()
                // Join with a comma separator.
                .join(",")
        }
    }
}


/// An item in the select parameter.
#[derive(Clone, Debug)]
pub struct SelectItem {
    /// The name of the column on the database.
    pub column_name: String,
    /// The name to serialize it as in the JSON results.
    pub rename_as: Option<String>,
    /// The Postgres type to cast it to before serializing it.
    pub cast_to: Option<String>,
}

impl SelectItem {
    /// Construct a new item for a select parameter.
    pub fn new(col_name: String) -> Self {
        Self {
            column_name: col_name,
            rename_as: None,
            cast_to: None,
        }
    }

    /// Rename a column in the select parameter.
    pub fn with_name(mut self, name: String) -> Self {
        self.rename_as = Some(name);
        self
    }

    /// Cast a column to a new type in a select parameter.
    pub fn cast_to(mut self, ty: String) -> Self {
        self.cast_to = Some(ty);
        self
    }
}

impl Into<String> for SelectItem {
    fn into(self) -> String {
        // Destructure self.
        let SelectItem {
            column_name,
            rename_as,
            cast_to
        } = self;

        // Match on options.
        match (rename_as, cast_to) {
            (None, None) => column_name,
            (Some(name), None) => format!("{}:{}", name, column_name),
            (None, Some(ty)) => format!("{}::{}", column_name, ty),
            (Some(name), Some(ty)) => format!("{}:{}::{}", name, column_name, ty)
        }
    }
}