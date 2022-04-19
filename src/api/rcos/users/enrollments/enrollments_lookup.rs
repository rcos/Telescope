//! RCOS API query to get enrollment record.

use std::string;

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/enrollments/enrollments_lookup.graphql",
    response_derives = "Debug,Clone,Serialize"
)]

pub struct EnrollmentsLookup;

impl EnrollmentsLookup{
    pub async fn get_by_id(semester_id: String) -> Result<enrollments_lookup::ResponseData, TelescopeError> {
        send_query::<Self>(enrollments_lookup::Variables { semester_id}).await
    }
}