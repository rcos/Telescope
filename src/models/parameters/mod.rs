//! Query parameters that can be used with the API.

use crate::models::parameters::pagination::PaginationParameter;
use crate::models::parameters::order::OrderParameter;

pub mod order;
pub mod pagination;

/// Parameters that are sent with queries to the central RCOS API.
#[derive(Clone, Debug, Serialize, Default)]
pub struct QueryParameters {
    /// Pagination of the results
    #[serde(flatten)]
    pub pagination: Option<PaginationParameter>,

    /// Ordering the results bu a specific column.
    #[serde(flatten)]
    pub ordering: Option<OrderParameter>,


}