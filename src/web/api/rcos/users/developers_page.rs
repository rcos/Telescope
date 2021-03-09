//! RCOS API query to get list of developers to display on the developers page.

// Aliases necessary for generated module's namespace.
use super::UserAccountType as user_account;
use super::UserRole as user_role;

/// Type representing GraphQL query to get a list of users and their
/// account associations.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/users/developers.graphql"
)]
pub struct Developers;

use developers::Variables;

impl Developers {
    /// Create the variables object to pass to the GraphQL query. This is just
    /// an empty object that instantiates the generated type.
    pub fn make_variables() -> Variables { Variables {} }
}
