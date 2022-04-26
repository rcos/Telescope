//! RCOS API query to get enrollment record.

use crate::api::rcos::send_query;
use crate::api::rcos::{prelude::*, search_strings::resolve_search_string};
use crate::error::TelescopeError;

/// The query returns 20 developers per page.
pub const PER_PAGE: u32 = 20;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/enrollments/user_enrollment_lookup.graphql",
    response_derives = "Debug,Clone,Serialize"
)]

pub struct UserEnrollmentLookup;

impl UserEnrollmentLookup {
    pub async fn get_by_id(
        page_num: u32,
        search: Option<String>,
        semester_id: String,
    ) -> Result<user_enrollment_lookup::ResponseData, TelescopeError> {
        send_query::<Self>(user_enrollment_lookup::Variables {
            limit: PER_PAGE as i64,
            offset: (PER_PAGE * page_num) as i64,
            search: resolve_search_string(search),
            semester_id: semester_id,
        })
        .await
    }
}
