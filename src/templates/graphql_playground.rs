use crate::web::Template;

/// The GraphQL Playground Page.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GraphQlPlaygroundPage {
    endpoint: String
}

impl GraphQlPlaygroundPage {
    /// Create a new playground page.
    pub fn for_endpoint(endpoint: impl Into<String>) -> Self {
        Self {endpoint: endpoint.into()}
    }
}

impl Template for GraphQlPlaygroundPage {
    const TEMPLATE_NAME: &'static str = "graphql_playground";
}