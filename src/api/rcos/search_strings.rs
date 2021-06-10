//! Utility functions for handling search strings before being sent to Hasura.

use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref SEARCH_REGEX: Regex = Regex::new(r"[@%\\]").unwrap();
}

/// Escape a search string by putting a back-slash before all
/// special characters (`_`, `%`, or `\`).
fn escape_search_string(search: &str) -> Cow<'_, str> {
    // Replace all instances of the matched regex with themself preceded
    // by a back-slash
    SEARCH_REGEX.replace_all(search, "\\$0")
}

/// Escape a search string and format with hasura regular expression characters
/// or produce the default (all-accepting) search string.
pub fn resolve_search_string(search: Option<String>) -> String {
    search
        // Escape the search string and surround it in `%`s to match on any sequence.
        .map(|s| format!("%{}%", escape_search_string(s.as_str())))
        // Default to match any user on no search string.
        .unwrap_or("%".into())
}
