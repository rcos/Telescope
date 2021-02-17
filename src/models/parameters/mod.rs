//! Query parameters that can be used with the API.

use crate::models::parameters::pagination::PaginationParameter;
use crate::models::parameters::order::OrderParameter;
use crate::models::parameters::select::SelectParameter;
use crate::models::parameters::filter::FilterParameter;

pub mod order;
pub mod pagination;
pub mod select;
pub mod filter;

/// Parameters that are sent with queries to the central RCOS API.
#[derive(Clone, Debug, Serialize, Default)]
pub struct QueryParameters {
    /// Pagination of the results
    #[serde(flatten)]
    pub pagination: Option<PaginationParameter>,

    /// Ordering the results bu a specific column.
    #[serde(flatten)]
    pub ordering: Option<OrderParameter>,

    /// Parameter to select columns. Defaults to all.
    #[serde(flatten)]
    pub select: SelectParameter,

    /// Parameter to filter results by value.
    #[serde(flatten)]
    pub filter: Option<FilterParameter>
}

impl QueryParameters {
    /// Serialize this object into a url endcoded string.
    pub fn url_encoded(&self) -> String {
        serde_urlencoded::to_string(self)
            .expect("Could not serialize query parameters")
    }
}