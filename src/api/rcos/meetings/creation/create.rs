//! GraphQL mutation to create a meeting.

use crate::api::rcos::{
    prelude::*,
    send_query
};
use crate::error::TelescopeError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/meetings/creation/create.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct CreateMeeting;

impl CreateMeeting {

}
