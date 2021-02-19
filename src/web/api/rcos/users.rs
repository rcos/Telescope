//! API interactions for RCOS users from the central RCOS API.


/// The valid user roles for all users in the RCOS database.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Student,
    Faculty,
    FacultyAdvisor,
    Alumn,
    External,
    ExternalMentor,
}

/// The valid account types for all user accounts stored in the RCOS database.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserAccountType {
    Rpi,
    Discord,
    Mattermost,
    GitHub,
    GitLab,
    BitBucket,
}

/// Type for usernames -- They are currently strings, but may change to UUIDs or
/// integers in the future.
pub type Username = String;

/// Data passed to the mutation to create a single user.
#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct CreateOneUser {
    username: Username,
    first_name: String,
    last_name: String,
    preferred_name: Option<String>,

}

//
// impl User {
//     /// The path on the API endpoint for the user table.
//     const PATH: &'static str = "users";
//
//     /// Store this user on the central database.
//     pub async fn create(&self) -> Result<(), TelescopeError> {
//         // Create the http client to communicate with the central RCOS API.
//         let http_client: Client = make_api_client(None);
//
//         info!("Adding user to database: {}", self.username);
//
//         // Send the request.
//         let response = http_client
//             .post(format!("{}/{}", api_endpoint(), Self::PATH))
//             .send_json(self)
//             .await
//             // Convert and propagate any errors.
//             .map_err(TelescopeError::api_query_error)?;
//
//         // Check the status code.
//         if response.status() != StatusCode::CREATED {
//             return Err(TelescopeError::ise("Could not add new user to the central RCOS database. \
//         Please contact a coordinator and file a GitHub issue."));
//         }
//         // Otherwise we were successful in creating a user.
//         Ok(())
//     }
//
//     /// Try to get a user from the database by their username
//     pub async fn get_by_username(username: impl Into<String>) -> Result<Option<Self>, TelescopeError> {
//         // Make an http client.
//         let http_client: Client = make_api_client(AUTHENTICATED_USER, ACCEPT_JSON);
//
//         // Convert the username.
//         let username: String = username.into();
//
//         info!("Finding user by username: {}", username);
//
//         // Construct query parameters.
//         let params: QueryParameters = QueryParameters {
//             filter: Some(FilterParameterRepr::comparison(
//                 "username".into(),
//                 ComparisonOperator::Equal,
//                 username).into()),
//             pagination: Some(PaginationParameter {
//                 limit: Some(1),
//                 offset: 0
//             }),
//             .. QueryParameters::default()
//         };
//
//         // Format the URL to query.
//         let url: String = format!("{}/{}?{}", api_endpoint(), Self::PATH, params.url_encoded());
//         info!("Querying API at {}", url);
//
//         let user: Option<User> = http_client
//             // Send request with query parameter for username filter.
//             .get(url)
//             .send()
//             .await
//             // Catch and propagate any errors.
//             .map_err(TelescopeError::api_query_error)?
//             // Convert to a list of users.
//             .json::<Vec<User>>()
//             .await
//             // Catch and propagate errors.
//             .map_err(TelescopeError::api_response_error)?
//             // The list should have one item if any.
//             .pop();
//
//         return Ok(user);
//     }
//
//
// }
//
// impl UserAccount {
//     const PATH: &'static str = "user_accounts";
//
//     /// Get a user account by a username and type.
//     pub async fn get_by_username_and_type(username: impl Into<String>, ty: &UserAccountType) -> Result<Option<Self>, TelescopeError> {
//         // Create http client.
//         let http_client: Client = make_api_client(AUTHENTICATED_USER, ACCEPT_JSON);
//
//         // Construct query parameters.
//         let params: QueryParameters = QueryParameters {
//             filter: Some(FilterParameterRepr::and(
//                 FilterParameterRepr::comparison(
//                     "username".into(),
//                     ComparisonOperator::Equal,
//                     username.into()
//                 ),
//                 FilterParameterRepr::comparison(
//                     "type".into(),
//                     ComparisonOperator::Equal,
//                     serde_json::to_string(ty)
//                         .expect("Couldn't serialize user account type")
//                 )
//             ).into()),
//             pagination: Some(PaginationParameter {
//                 limit: Some(1),
//                 offset: 0
//             }),
//             .. QueryParameters::default()
//         };
//
//         // Format URL.
//         let url: String = format!("{}/{}?{}", api_endpoint(), Self::PATH, params.url_encoded());
//         info!("Querying API for user account at {}", url);
//
//         // Query API.
//         let response: Option<UserAccount> = http_client
//             .get(url)
//             .send()
//             .await
//             .map_err(TelescopeError::api_query_error)?
//             .json::<Vec<UserAccount>>()
//             .await
//             .map_err(TelescopeError::api_response_error)?
//             .pop();
//
//         Ok(response)
//     }
// }
