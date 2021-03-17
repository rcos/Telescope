//! Mutation to insert or update a user account record.

// Namespace items for generated code
use crate::web::api::rcos::users::UserAccountType as user_account;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/accounts/upsert.graphql"
)]
pub struct UpsertUserAccount;

use upsert_user_account::{
    Variables,
    ResponseData
};

impl UpsertUserAccount {
    /// Make the variables for a user account upsert mutation.
    pub fn make_variables() -> Variables {
        unimplemented!()
    }
}