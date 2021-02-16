//! Query parameters that can be used with the API.

use crate::models::parameters::pagination::PaginationParameter;
use crate::models::parameters::order::OrderParameter;
use crate::models::parameters::select::SelectParameter;

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


}