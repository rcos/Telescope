//! Models and functionality for the GraphQL interactions with the central
//! RCOS API.

use serde_json::Value;

/// The GraphQL request type.
#[derive(Serialize, Clone, Debug)]
pub struct GraphQLPostRequest {
    query: String
}

pub struct GraphQLResponse {
    data: Value,
    
}