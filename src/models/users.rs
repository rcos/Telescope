use crate::schema::users;
use uuid::Uuid;

use argon2::{self, Config};
use crate::web::api::PasswordRequirements;

#[derive(Insertable, Queryable, Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
#[table_name = "users"]
#[graphql(description = "A telescope user.")]
pub struct User {
    /// User's universally unique identifier
    pub id: Uuid,
    /// User's name.
    pub name: String,
    /// Optionally, a link to the user's avatar (profile picture).
    ///
    /// Use the default statically served avatar photo if this is not available.
    pub avi_location: Option<String>,
    /// The user's bio. This is in commonmark markdown format.
    pub bio: String,
    /// A link to the user's Github
    pub github_link: Option<String>,
    // FIXME: Discord & Mattermost integration.
    /// The user's discord or mattermost chat handle.
    /// (Since RCOS transfered to discord, this is in limbo)
    pub chat_handle: Option<String>,
    /// Is this user a telescope admin.
    pub sysadmin: bool,
    /// The hashed user password.
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
            bio: String::default(),
            github_link: None,
            chat_handle: None,
            sysadmin: false,
            hashed_pwd
        })
    }

    /// Format a uuid into a lowercase hyphenated string.
    pub fn format_uuid(id: Uuid) -> String {
        id.to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string()
    }

    /// Format the associated user id into a string.
    pub fn id_str(&self) -> String {
        Self::format_uuid(self.id)
    }
}
