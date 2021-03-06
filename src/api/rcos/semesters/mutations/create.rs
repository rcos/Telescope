//! GraphQL mutation to create a semester in the RCOS dataabse.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;
use chrono::NaiveDate;

/// Type representing GraphQL mutation to create semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/semesters/mutations/create.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CreateSemester;

impl CreateSemester {
    /// Create a semester. Return the semester ID or an error.
    pub async fn execute(
        id: String,
        title: String,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<String, TelescopeError> {
        return send_query::<Self>(create_semester::Variables {
            id,
            title,
            start,
            end,
        })
        .await
        // Extract semester ID.
        .map(|r| r.insert_semesters_one.unwrap().semester_id);
    }
}
