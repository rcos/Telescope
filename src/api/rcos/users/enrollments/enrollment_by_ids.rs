use crate::error::TelescopeError;
use crate::api::rcos::{prelude::*, send_query};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/enrollments/enrollment_by_ids.graphql",
    response_derives = "Debug,Clone,Serialize"
)]

pub struct EnrollmentByIds;

impl EnrollmentByIds {
    pub async fn get(
        user_id: uuid,
        semester_id: String,
        ) -> Result<enrollment_by_ids::ResponseData, TelescopeError> {
        send_query::<Self>(enrollment_by_ids::Variables {
            semester_id,
            user_id,
        })
        .await
    }
}
