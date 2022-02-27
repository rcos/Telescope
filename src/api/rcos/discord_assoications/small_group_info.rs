use crate::api::rcos::discord_assoications::small_group_info::curr_small_groups::ResponseData;
use crate::api::rcos::discord_assoications::small_group_info::curr_small_groups::Variables;
use crate::api::rcos::{prelude::*, search_strings::resolve_search_string, send_query};
use crate::error::TelescopeError;
use chrono::Utc;

/// Projects per page.
const PER_PAGE: u32 = 20;

/// GraphQL query to get projects with enrollments in an ongoing semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_assoications/small_groups.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CurrSmallGroups;
impl CurrSmallGroups {
    pub async fn get(page: u32, search: Option<String>) -> Result<ResponseData, TelescopeError> {
        send_query::<Self>(Variables {
            offset: (PER_PAGE * page) as i64,
            search: resolve_search_string(search),
            now: Utc::today().naive_utc(),
        })
        .await
    }
}
