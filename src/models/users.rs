use crate::schema::users;
use uuid::Uuid;

use argon2::{self, Config};
use crate::web::api::PasswordRequirements;

#[derive(Insertable, Queryable, Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
#[table_name = "users"]
#[graphql(description = "A telescope user.")]
pub struct User {
    #[graphql(description = "Universally unique user identifier")]
    pub id: Uuid,
    pub name: String,
    pub avi_location: Option<String>,
    pub bio: Option<String>,
    pub github: Option<String>,
    pub chat_handle: Option<String>,
    /// The hashed user password.
    #[serde(skip)]
    #[graphql(skip)]
    pub hashed_pwd: String,
}

impl User {
    /// Number of bytes in a password hash
    const HASH_LENGTH: u32 = 32;

    /// Create the argon config we use for telescope.
    fn make_argon_config<'a>() -> Config<'a> {
        let mut argon_cfg = Config::default();
        argon_cfg.hash_length = Self::HASH_LENGTH;

        // Strongest argon version
        argon_cfg.variant = argon2::Variant::Argon2id;
        argon_cfg.version = argon2::Version::Version13;

        // two lane parallel
        argon_cfg.lanes = 2;
        argon_cfg.thread_mode = argon2::ThreadMode::Parallel;

        argon_cfg
    }

    /// Create a new user from a name and a password. Randomly generate a UUID.
    /// Do not set any user info yet. Fail if password does nto meet requirements.
    pub fn new<T: Into<String>>(name: T, password: &str) -> Result<Self, PasswordRequirements> {
        let reqs = PasswordRequirements::for_password(password);

        if !reqs.are_satisfied() {
           return Err(reqs);
        }

        let uuid = Uuid::new_v4();

        let hashed_pwd = argon2::hash_encoded(
            password.as_bytes(),
            &uuid.as_bytes()[..],
            &Self::make_argon_config()
        ).unwrap();

        Ok(Self {
            id: uuid,
            name: name.into(),
            avi_location: None,
            bio: None,
            github: None,
            chat_handle: None,
            hashed_pwd
        })
    }
}
