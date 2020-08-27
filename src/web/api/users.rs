use crate::schema::users;
use uuid::Uuid;

use actix_web::Error;

use super::root::ApiContext;

#[derive(Insertable, Queryable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub avi_location: Option<String>,
    /// The hashed user password.
    #[serde(skip)]
    pub hashed_pwd: String,
}

#[juniper::object(Context = ApiContext)]
#[graphql(description = "An RCOS user")]
impl User {
    #[graphql(description = "Universally unique user identifier")]
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    // Create a new user from a name and a password. Randomly generate a UUID. Do not set profile
    // picture yet.
    //#[graphql(skip)]
    // pub fn new<T: Into<String>>(name: T, password: &str) -> Result<Self, bcrypt::BcryptError> {
    //     let uuid = Uuid::new_v4();
    //
    //     let hashed_pwd = bcrypt::hash_with_salt(
    //         password,
    //         bcrypt::DEFAULT_COST,
    //         uuid.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()).as_bytes()
    //     )
    //         .map(|hp| hp.format_for_version(bcrypt::Version::TwoA))?;
    //
    //     Ok(Self {
    //         id: uuid,
    //         name: name.into(),
    //         avi_location: None,
    //         hashed_pwd
    //     })
    // }
}
