//! RCOS API mutation to delete a user

use crate::api::rcos::{send_query, prelude::*};
use crate::error::TelescopeError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/delete.graphql"
)]
pub struct DeleteUser;

use delete_user::{ResponseData, Variables};

impl DeleteUser {
    pub async fn execute(user_id: uuid) -> Result<ResponseData, TelescopeError> {
        send_query::<Self>(Variables { user_id }).await
    }
}
