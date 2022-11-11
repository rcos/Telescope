//! GraphQL query for info about the current semester.

use crate::api::rcos::{prelude::*, send_query};
use crate::error::TelescopeError;
use chrono::prelude::*;

/// Type representing GraphQL query for current semester data.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/semesters/current/info.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CurrentSemesters;

impl CurrentSemesters{
    pub async fn get() -> Result<current_semesters::ResponseData, TelescopeError> {
        send_query::<Self>(current_semesters::Variables{
            now: Utc::now().naive_utc().date(),
        })
        .await
    }
}
