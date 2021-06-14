//! GraphQL query to get a single semester record by ID.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing GraphQL mutation to make changes to a semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/semesters/get_by_id.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Semester;

impl Semester {
    /// Get a semester record by ID.
    pub async fn get_by_id(
        id: String,
    ) -> Result<Option<semester::SemesterSemestersByPk>, TelescopeError> {
        send_query::<Self>(semester::Variables { id })
            .await
            .map(|data| data.semesters_by_pk)
    }
}
