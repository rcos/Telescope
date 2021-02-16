//! Pagination parameters.

/// Pagination parameter.
#[derive(Copy, Clone, Debug, Serialize, Default)]
pub struct PaginationParameter {
    /// The offset into the source dataset.
    pub offset: u64,
    /// The maximum number of records that should be returned from the query.
    pub limit: Option<u64>
}
