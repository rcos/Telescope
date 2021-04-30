//! Module for Landing Page statistics query and data extraction.

use crate::api::rcos::{prelude::*, send_query};
use crate::error::TelescopeError;
use chrono::Utc;

/// GraphQL Query for landing page statistics.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/stats/landing_page.graphql",
    response_derives = "Serialize"
)]
pub struct LandingPageStatistics;

use self::landing_page_statistics::{ResponseData, Variables};

impl LandingPageStatistics {
    /// Get the landing page statistics from the RCOS API.
    pub async fn get() -> Result<ResponseData, TelescopeError> {
        return send_query::<Self>(Variables {
            now: Utc::today().naive_utc(),
        })
        .await;
    }
}
