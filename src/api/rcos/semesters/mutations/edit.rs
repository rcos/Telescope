//! Edit mutation on semesters.

use crate::api::rcos::prelude::*;
use chrono::NaiveDate;
use crate::error::TelescopeError;
use crate::api::rcos::send_query;

/// Type representing GraphQL mutation to make changes to a semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/semesters/mutations/edit.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct EditSemester;

impl EditSemester {
    /// Send a semester edit mutation. Return a semester ID if there was a semester found and edited.
    pub async fn execute(id: String, new_title: String, new_start: NaiveDate, new_end: NaiveDate)
        -> Result<Option<String>, TelescopeError>
    {
        send_query::<Self>(edit_semester::Variables {
            semester_id: id,
            set_title: Some(new_title),
            set_start: Some(new_start),
            set_end: Some(new_end),
        })
            .await
            .map(|data| data.update_semesters_by_pk.map(|obj| obj.semester_id))
    }
}
