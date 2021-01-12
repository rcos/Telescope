///! The GraphQL Playground Page.
use crate::templates::Template;

const TEMPLATE_NAME: &'static str = "graphql_playground";

/// Create a new playground page template.
pub fn for_endpoint(endpoint: impl Into<String>) -> Template {
    Template::new(TEMPLATE_NAME).field("endpoint", endpoint.into())
}
