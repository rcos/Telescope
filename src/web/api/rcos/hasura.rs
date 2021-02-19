//! Models for structs returned by Hasura specifically.

use serde::Deserialize;
use std::fmt::Debug;

/// Many results from hasura's mutations have a field for the number of
/// affected rows.
#[derive(Clone, Deserialize, Debug)]
pub struct MutationResult<T: Debug + Clone> {
    /// How many rows of the database were affected
    affected_rows: u64,
    /// The return value of the mutation.
    returning: Vec<T>
}
