//! Edit mutation on semesters.

use crate::api::rcos::prelude::*;

/// Type representing GraphQL mutation to make changes to a semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/semesters/mutations/edit.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct EditSemester;
