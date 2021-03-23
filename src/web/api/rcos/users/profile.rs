//! Profile query.

use chrono::{DateTime, Utc};
use crate::error::TelescopeError;
use crate::templates::user::profile::TargetUser;

// Namespaced types for generated code
use crate::web::api::rcos::users::{UserAccountType as user_account, UserRole as user_role};

// Ignore the compiler warning this style would generate.
#[allow(nonstandard_style)]
type timestamptz = DateTime<Utc>;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rcos/schema.json",
    query_path = "graphql/rcos/users/profile.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct Profile;

// import generated types.
use profile::{
    Variables,
    ResponseData,
    ProfileUsersByPk
};
use crate::web::api::rcos::send_query;
use std::collections::HashMap;

impl Profile {
    /// Get the profile data for a given username.
    pub async fn for_user(username: String) -> Result<ResponseData, TelescopeError> {
        send_query::<Self>(Variables { username }).await
    }
}

impl Into<TargetUser> for ProfileUsersByPk {
    fn into(self) -> TargetUser {
        // Join the names.
        let name: String = format!("{} {}", self.first_name, self.last_name);
        // Convert the creation date to a string.
        let created_at: String = self.created_at
            // In local timezone
            .naive_local()
            // Get just the date
            .date()
            // Format Month Day Year (e.g. July 1, 2020)
            .format("%B %_d, %Y")
            // Convert to string.
            .to_string();
        // Convert the user_accounts list into a map over the user accounts.
        let accounts: HashMap<user_account, String> = self.user_accounts
            // Iterate over user accounts.
            .into_iter()
            // Convert each item to a tuple
            .map(|acc| (acc.type_, acc.account_id))
            // Collect the list of tuples into a hashmap
            .collect();

        TargetUser {
            name,
            // cohort: self.cohort,
            created_at,
            accounts
        }
    }
}
