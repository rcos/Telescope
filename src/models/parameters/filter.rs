//! Parameter for filtering API results by value.
//! See https://postgrest.org/en/v7.0.0/api.html#horizontal-filtering-rows for
//! more information.

use std::collections::HashMap;
use std::ops::Not;

/// A parameter to filter the API results by value.
#[derive(Clone, Debug, Serialize)]
pub struct FilterParameter {
    /// The key and value of the filter parameter to the query.
    #[serde(flatten)]
    inner: HashMap<String, String>
}

/// Postgres comparison operators. These all display as the text that should be
/// used in the API query string.
#[derive(Copy, Clone, Debug, Display)]
pub enum ComparisonOperator {
    #[display(fmt = "eq")]
    Equal,
    #[display(fmt = "gt")]
    GreaterThan,
    #[display(fmt = "lt")]
    LessThan,
    #[display(fmt = "gte")]
    GreaterThanOrEqual,
    #[display(fmt = "lte")]
    LessThanOrEqual,
    #[display(fmt = "neq")]
    NotEqual,
    #[display(fmt = "is")]
    Is,
}

/// Enumeration of representations for filter parameters. These are used to
/// build the actual filter parameter. These can be built into trees to
/// represent increasingly complex filters.
#[derive(Clone, Debug)]
pub enum FilterParameterRepr {
    /// Negation of another filter parameter.
    Not(Box<FilterParameterRepr>),

    /// Comparison filter.
    Compares {
        /// The column to filter.
        column: String,
        /// The operation.
        op: ComparisonOperator,
        /// The value to reference in the operation.
        value: String,
    },

    /// Join over two filters.
    And(Box<FilterParameterRepr>, Box<FilterParameterRepr>),

    /// Disjoin over two filters.
    Or(Box<FilterParameterRepr>, Box<FilterParameterRepr>),
}


impl FilterParameterRepr {
    /// Construct a join over two filters.
    pub fn and(a: FilterParameterRepr, b: FilterParameterRepr) -> Self {
        Self::And(Box::new(a), Box::new(b))
    }

    /// Construct a disjoin over two filters.
    pub fn or(a: FilterParameterRepr, b: FilterParameterRepr) -> Self {
        Self::Or(Box::new(a), Box::new(b))
    }

    /// Construct a new comparison filter. There will be issues if the value has a comma in it or is
    /// otherwise not properly escaped.
    pub fn comparison(col_name: String, comparison: ComparisonOperator, value: String) -> Self {
        Self::Compares {
            column: col_name,
            op: comparison,
            value
        }
    }

    /// Convert this filter into a key value representation to add to the
    /// query string.
    fn convert(self) -> (String, String) {
        match self {
            // Negation of a filter.
            Self::Not(f1) => {
                match *f1 {
                    // Two negations cancel out.
                    Self::Not(f2) => f2.convert(),
                    // On comparisons, the not is added to the operator.
                    Self::Compares { column, op, value } =>
                        (column, format!("not.{}.{}", op, value)),
                    // Otherwise the not is added to the key.
                    other => {
                        let (k, v) = other.convert();
                        (format!("not.{}", k), v)
                    }
                }
            }

            // Comparison filter
            Self::Compares {column, op, value} =>
                (column, format!("{}.{}", op, value)),

            // Join over two filters
            Self::And(a, b) =>
                (format!("and"), format!("({},{})", a.sublevel_convert(), b.sublevel_convert())),

            // Disjoin of two filters
            Self::Or(a, b) =>
                (format!("or"), format!("({},{})", a.sublevel_convert(), b.sublevel_convert())),
        }
    }

    /// Special conversion function for converting when the filter is not top-level.
    fn sublevel_convert(self) -> String {
        match self {
            Self::And(a, b) =>
                format!("and({},{})", a.sublevel_convert(), b.sublevel_convert()),

            Self::Or(a,b) =>
                format!("or({}, {})", a.sublevel_convert(), b.sublevel_convert()),

            Self::Not(i) => match *i {
                // Double negation cancels
                Self::Not(i2) => i2.sublevel_convert(),
                // On comparisons the not must be before the operator
                Self::Compares {column, op, value} =>
                    format!("{}.not.{}.{}", column, op, value),
                // Otherwise the not just precedes the sub-level conversion.
                other=> format!("not.{}", other.sublevel_convert()),
            }

            Self::Compares {column, op, value} =>
                format!("{}.{}.{}", column, op, value)
        }
    }
}

impl Not for FilterParameterRepr {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

impl Into<FilterParameter> for FilterParameterRepr {
    fn into(self) -> FilterParameter {
        let mut map = HashMap::new();
        let (k, v) = self.convert();
        map.insert(k, v);

        return FilterParameter { inner: map };
    }
}
