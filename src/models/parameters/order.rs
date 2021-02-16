//! Parameter to order API results.

/// Parameter to postgres to order by a specific column.
#[derive(Clone, Debug, Serialize)]
pub struct OrderParameter {
    /// The order string. This is in the format
    /// `column_name.[asc|desc].[nullsfirst|nullslast]`.
    order: String
}

impl OrderParameter {
    // Constant formatting strings.
    const ASCENDING: &'static str = "asc";
    const DESCENDING: &'static str = "desc";
    const NULLS_FIRST: &'static str = "nullsfirst";
    const NULLS_LAST: &'static str = "nullslast";

    /// Construct an order parameter string.
    fn new(col_name: String, order: &'static str, nulls: &'static str) -> Self {
        Self {
            order: format!("{}.{}.{}", col_name, order, nulls)
        }
    }

    /// Sort by a given column ascending with nulls first.
    pub fn asc_nullsfirst(col_name: impl Into<String>) -> Self {
        Self::new(col_name.into(), Self::ASCENDING, Self::NULLS_FIRST)
    }

    /// Sort a given column ascending with nulls last.
    pub fn asc_nullslast(col_name: impl Into<String>) -> Self {
        Self::new(col_name.into(), Self::ASCENDING, Self::NULLS_LAST)
    }

    /// Sort a given column descending with nulls first.
    pub fn desc_nullsfirst(col_name: impl Into<String>) -> Self {
        Self::new(col_name.into(), Self::DESCENDING, Self::NULLS_FIRST)
    }

    /// Sort a given column descending with nulls last.
    pub fn desc_nullslast(col_name: impl Into<String>) -> Self {
        Self::new(col_name.into(), Self::DESCENDING, Self::NULLS_LAST)
    }
}
