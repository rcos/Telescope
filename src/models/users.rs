use crate::{
    models::{emails::Email, password_requirements::PasswordRequirements},
    schema::users,
    util::handle_blocking_err,
    web::{api::graphql::ApiContext, DbConnection},
};

use actix_web::web::block;
use argon2::{self, Config};
use chrono::{DateTime, Utc};
use juniper::{FieldError, FieldResult, Value};
use uuid::Uuid;

/// A telescope user.
#[derive(Insertable, Queryable, Debug, Clone, Serialize, Deserialize, Associations)]
#[table_name = "users"]
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
    // FIXME: Discord & Mattermost integration.
    /// A link to the user's Github
    pub github_link: Option<String>,
    /// The user's discord or mattermost chat handle.
    /// (Since RCOS transfered to discord, this is in limbo)
    pub chat_handle: Option<String>,
    /// Is this user a telescope admin.
    pub sysadmin: bool,
    /// The hashed user password.
    pub hashed_pwd: String,
    /// The moment that the account was created.
    pub account_created: DateTime<Utc>,
}

/// An RCOS member.
#[graphql_object(Context = ApiContext)]
impl User {
    /// The user's unique identifier.
    fn id(&self) -> Uuid {
        self.id
    }

    /// The user's name.
    fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The profile picture url of the user.
    fn avi_location(&self) -> &Option<String> {
        &self.avi_location
    }

    /// The bio of the user.
    fn bio(&self) -> &str {
        self.bio.as_str()
    }

    /// Is this user a sysadmin.
    fn is_sysadmin(&self) -> bool {
        self.sysadmin
    }

    /// When the account was created.
    fn account_created(&self) -> DateTime<Utc> {
        self.account_created
    }

    // Github links and chat handles are not public as they are not stable API
    // They will be replaced when these services are integrated

    // passwords are out of the public API for obvious reasons.

    // computed fields below

    // this code may block, but since its only executed by juniper
    // it should always be executed on an async thread pool anyways.
    /// Public emails of this user.
    fn emails(&self, ctx: &ApiContext) -> FieldResult<Vec<Email>> {
        use crate::schema::emails;
        use diesel::prelude::*;

        let conn = ctx.get_db_conn()?;

        let db_results: QueryResult<Vec<(User, Email)>> = users::table
            .inner_join(emails::table)
            .filter(users::dsl::id.eq(self.id).and(emails::dsl::is_visible))
            .load(&conn);

        db_results
            .map_err(|e| {
                error!("Could not query database: {}", e);
                FieldError::new("Could not query database.", Value::null())
            })
            .map(|v: Vec<(User, Email)>| v.into_iter().map(|(u, e)| e).collect())
    }
}

/// Rust only user operations and constants.
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
        let reqs: PasswordRequirements = PasswordRequirements::for_password(password);

        if !reqs.satisfied() {
            return Err(reqs);
        }

        let uuid = Uuid::new_v4();

        let hashed_pwd = argon2::hash_encoded(
            password.as_bytes(),
            &uuid.as_bytes()[..],
            &Self::make_argon_config(),
        )
        .unwrap();

        Ok(Self {
            id: uuid,
            name: name.into(),
            avi_location: None,
            bio: String::default(),
            github_link: None,
            chat_handle: None,
            sysadmin: false,
            hashed_pwd,
            account_created: Utc::now(),
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

    /// Get a user from the database by user id asynchronously.
    ///
    /// Return none if user is not found.
    pub async fn get_from_db_by_id(conn: DbConnection, uid: Uuid) -> Option<User> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;

        block(move || users.find(uid).first::<User>(&conn).optional())
            .await
            .map_err(|e| {
                error!("Could not get user from database: {}", e);
                e
            })
            .unwrap()
    }

    // TODO: Test?
    /// Get a user's emails from the database. Return empty vector if there are no
    /// emails found. Returned emails will be sorted by visibility, and then
    /// alphabetically.
    pub async fn get_emails_from_db_by_id(conn: DbConnection, uid: Uuid) -> Vec<Email> {
        use crate::schema::emails::dsl::*;
        use diesel::prelude::*;

        block::<_, Vec<Email>, _>(move || {
            emails
                .filter(user_id.eq(uid))
                .order((is_visible.asc(), email.asc()))
                .load(&conn)
        })
        .await
        .map_err(|e| {
            error!("Could not query database: {}", e);
            e
        })
        .unwrap()
    }

    /// See the get_emails_from_db_by_id
    pub async fn get_emails_from_db(&self, conn: DbConnection) -> Vec<Email> {
        User::get_emails_from_db_by_id(conn, self.id).await
    }

    /// Store the user in the database. On conflict, return error.
    pub async fn store(self, conn: DbConnection) -> Result<(), String> {
        block::<_, usize, _>(move || {
            use crate::schema::users::dsl::*;
            use diesel::prelude::*;
            diesel::insert_into(users).values(&self).execute(&conn)
        })
        .await
        .map_err(|e| handle_blocking_err(e, "Could not add user to database."))
        .map(|n| trace!("Added {} user(s) to database.", n))
    }
}
