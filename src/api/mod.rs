//! Different API services that Telescope consumes.

use crate::error::TelescopeError;
use graphql_client::Response;

pub mod discord;
pub mod github;
pub mod rcos;

/// Handle a response from a GraphQL API. Convert any errors as necessary and
/// extract the returned data if possible.
fn handle_graphql_response<T>(
    api_name: &'static str,
    response: Response<T>,
) -> Result<T, TelescopeError> {
    // Match on the response structure.
    match response {
        // If errors and data are both non-null
        Response {
            errors: Some(errs),
            data: Some(rdata),
        } => {
            if errs.is_empty() {
                // If there are no errors return the data.
                Ok(rdata)
            } else {
                // If there are errors, return those with the API name.
                Err(TelescopeError::GraphQLError {
                    platform: api_name.to_string(),
                    errors: errs,
                })
            }
        }

        // If no errors, return the data.
        Response {
            errors: None,
            data: Some(rdata),
        } => Ok(rdata),

        // If just errors, return those.
        Response {
            errors: Some(errs),
            data: None,
        } => {
            if errs.is_empty() {
                panic!("Central GraphQL API returned a response with no errors or data.");
            } else {
                Err(TelescopeError::GraphQLError {
                    platform: api_name.to_string(),
                    errors: errs,
                })
            }
        }

        // Panic on None of either.
        Response {
            errors: None,
            data: None,
        } => panic!("Central GraphQL API responded with no errors or data."),
    }
}
