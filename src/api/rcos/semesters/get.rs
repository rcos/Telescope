//! GraphQL query to get semester records.

use crate::api::rcos::{prelude::*, send_query};
use crate::error::TelescopeError;

/// Type representing GraphQL query for current semester data.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/semesters/get.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Semesters;

/// Semester records to get per page.
pub const PER_PAGE: u32 = 20;

impl Semesters {
    /// Get semester records (using a zero indexed page number)
    pub async fn get(page_num: u32) -> Result<semesters::ResponseData, TelescopeError> {
        send_query::<Self>(semesters::Variables {
            limit: PER_PAGE as i64,
            offset: (page_num*PER_PAGE) as i64
        }).await
    }
}
