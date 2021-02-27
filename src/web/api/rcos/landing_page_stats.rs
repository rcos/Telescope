//! Module for Landing Page statistics query and data extraction.

/// GraphQL Query for landing page statistics.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/stats/landing_page.graphql",
)]
pub struct LandingPageStatistics;

// Re-export variable and response types.
pub use self::landing_page_statistics::{
    Variables as LandingPageStatsVars,
    ResponseData as LandingPageStatsResponse
};

impl LandingPageStatsResponse {
    /// Extract the total number of students from the GraphQL response.
    pub fn total_students(&self) -> Option<i64> {
        self.total_students
            .aggregate.as_ref()?
            .count
    }

    /// Extract the total number of projects from the GraphQL response.
    pub fn total_projects(&self) -> Option<i64> {
        self.total_projects
            .aggregate.as_ref()?
            .count
    }

    /// Extract the current semester title from the GraphQL response.
    pub fn current_semester(&self) -> Option<String> {
        self.current_semester
            // The response will have at most 1 semester.
            .first()
            .map(|semester| semester.title.clone())
    }

    /// Extract the number of students this semester from the GraphQL response.
    pub fn current_students(&self) -> Option<i64> {
        self.current_semester
            .first()?
            .current_students
            .aggregate.as_ref()?
            .count
    }

    /// Extract the number of current projects this semester from the GraphQL response.
    pub fn current_projects(&self) -> Option<i64> {
        self.current_semester
            .first()?
            .current_projects
            .aggregate.as_ref()?
            .count
    }
}
