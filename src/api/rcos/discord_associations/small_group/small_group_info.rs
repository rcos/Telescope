use crate::api::rcos::discord_associations::small_group::small_group_info::curr_small_groups::ResponseData;
use crate::api::rcos::discord_associations::small_group::small_group_info::curr_small_groups::Variables;
use crate::api::rcos::{prelude::*, search_strings::resolve_search_string, send_query};
use crate::error::TelescopeError;
use chrono::Utc;

/// Projects per page.
const PER_PAGE: u32 = 20;

/// GraphQL query to get projects with enrollments in an ongoing semester.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_associations/small_group/small_groups.graphql",
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
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/discord_associations/small_group/find_small_group.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct FindCurrSmallGroup;
impl FindCurrSmallGroup{
    pub async fn get_by_id(id: i64) -> Result<find_curr_small_group::ResponseData, TelescopeError> {
        send_query::<Self>(find_curr_small_group::Variables {
            id: id,
            now: Utc::today().naive_utc(),
        })
        .await
    }
}

