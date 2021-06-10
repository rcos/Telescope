//! Template model for pagination bar.

use std::num::NonZeroU64;

/// Pagination template fields.
#[derive(Serialize, Copy, Clone, Debug)]
pub struct PaginationInfo {
    // first field excluded, first page is always page 1
    /// Flag to include left side ellipsis/separator.
    left_sep: bool,

    /// Previous page number (current - 1).
    prev: u64,

    /// Current page number.
    current: NonZeroU64,

    /// Next page number (current + 1).
    next: u64,

    /// Flag to include right side ellipsis/separator
    right_sep: bool,

    /// Last page number.
    last: u64,
}

impl PaginationInfo {
    /// Create a new pagination info object if there are enough results to
    /// require multiple pages.
    fn new_inner(items: u64, per_page: u64, current_page: NonZeroU64) -> Option<Self> {
        // Calculate the number of pages required.
        let pages_required: u64 = items / per_page + 1;

        if pages_required <= 1 {
            // No pagination if there is only one page.
            None
        } else {
            Some(PaginationInfo {
                // include left separator if there are pages between
                left_sep: (current_page.get() - 1) > 2,

                // If we're currently on the first page this will be hidden.
                prev: current_page.get() - 1,

                current: current_page,

                // If we're currently on the last page this will be hidden.
                next: current_page.get() + 1,

                // similar with right separator,
                right_sep: current_page.get() + 1 < pages_required - 1,

                last: pages_required,
            })
        }
    }

    /// Create a new pagination object with 1 indexed page numbers.
    /// Return [`Option::None`] if there is only one page.
    /// ## Panics:
    /// - If the `current_page` is 0.
    pub fn new(items: u64, per_page: u64, current_page: u64) -> Option<Self> {
        let converted = NonZeroU64::new(current_page).expect("current_page > 0");
        Self::new_inner(items, per_page, converted)
    }
}
