//! Meeting edit mutation and host selection query.

use crate::api::rcos::prelude::*;
use crate::api::rcos::send_query;
use crate::error::TelescopeError;

/// Type representing GraphQL enrollment edit mutation.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/enrollments/edit_enrollment.graphql",
    response_derives = "Debug,Copy,Clone,Serialize"
)]
pub struct EditEnrollment;

impl EditEnrollment{
    pub async fn execute(vars: edit_enrollment::Variables) -> Result<Option<uuid>, TelescopeError>{
       send_query::<Self>(vars)
           .await
           .map(|response| response.update_enrollments_by_pk.map(|obj| obj.user_id))
    }
}
