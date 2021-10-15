//! GraphQL query to get semester records.

use crate::api::rcos::{prelude::*, send_query};
use crate::error::TelescopeError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/enrollments/for_user.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Enrollments;

pub const PER_PAGE: u32 = 20;

impl Enrollments {
    pub async fn get(username: String, page_num: u32) -> Result<enrollments::ResponseData, TelescopeError> {
        send_query::<Self>(enrollments::Variables {
            username,
            limit: PER_PAGE as i64,
            offset: (page_num * PER_PAGE) as i64,
        })
        .await
    }
}

impl enrollments::ResponseData {
    pub fn enrollment_count(&self) -> Option<i64> {
        Some(self.enrollments_aggregate.aggregate.as_ref()?.count?)
    }
}
