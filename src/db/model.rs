//! Model of db entities

use uuid::Uuid;
use sha2::{Sha256, Digest};
use crate::schema::users;

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "users"]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub avi_location: Option<String>,
    pub hashed_pwd: String
}

impl User {
    /// Create a new user from a name and a password. Randomly generate a UUID. Do not set profile
    /// picture yet.
    pub fn new(name: impl Into<String>, password: &str) -> Self {
        let uuid = format!("{}", Uuid::new_v4().to_hyphenated());

        let mut hasher = Sha256::default();
        hasher.update(uuid.as_str());
        hasher.update(password);
        let hashed_pwd = format!("{:x}", hasher.finalize());

        Self {
            uuid,
            name: name.into(),
            avi_location: None,
            hashed_pwd
        }
    }
}

