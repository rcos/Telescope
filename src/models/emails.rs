use crate::{
    models::users::User,
    schema::emails,
    util::handle_blocking_err,
    web::{api::graphql::ApiContext},
    util::DbConnection,
};
use actix_web::web::block;
use juniper::{FieldError, FieldResult, Value};
use lettre::EmailAddress;
use uuid::Uuid;
use crate::app_data::AppData;
use crate::error::TelescopeError;

/// Field structure must match that in the SQL migration.
/// (for diesel reasons it seems)
#[derive(Clone, Serialize, Deserialize, Insertable, Queryable, Debug, Associations)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "emails"]
pub struct Email {
    /// The email
    pub email: String,
    /// is this email displayed on the website publicly?
    pub is_visible: bool,
    /// User id of associated user
    pub user_id: Uuid,
}

/// GraphQL operations on emails.
#[graphql_object(Context = ApiContext)]
impl Email {
    /// The email address
    fn address(&self) -> &str {
        self.email.as_str()
    }

    // this code may block, but since its only executed by juniper
    // it should always be executed on an async thread pool anyways.
    /// The user associated with this email address.
    fn user(&self, ctx: &ApiContext) -> FieldResult<User> {
        use crate::schema::users;
        use diesel::prelude::*;

        let conn = ctx.get_db_conn()?;

        let results: QueryResult<Vec<(Email, User)>> = emails::table
            .inner_join(users::table)
            .filter(emails::dsl::email.eq(self.email.as_str()))
            .limit(1)
            .load(&conn);

        results
            .map_err(|e| {
                error!("Could not query database: {}", e);
                FieldError::new("Could not query database.", Value::null())
            })?
            .pop()
            .ok_or(FieldError::new(
                "Could not find associated user.",
                Value::null(),
            ))
            .map(|(e, u)| u)
    }
}

impl Email {
    /// Create a new email object. Return none if email does not
    /// match regex.
    pub fn new<T: Into<String>>(user_id: Uuid, email: T) -> Option<Self> {
        EmailAddress::new(email.into())
            .map_err(|e| {
                error!("Email malformed: {}", e);
            })
            .map(|email: EmailAddress| Self {
                user_id,
                email: email.to_string(),
                is_visible: false,
            })
            .ok()
    }

    /// Create a new email object. This will not fail since the syntactic
    /// validity of the email is checked by the EmailAddress type.
    pub fn new_prechecked(user_id: Uuid, email: EmailAddress) -> Self {
        Self {
            user_id,
            email: email.to_string(),
            is_visible: false,
        }
    }

    /// Try to get a user based on an email from the database.
    ///
    /// Returns None if the user was not found or if there was an issues accessing the database.
    pub async fn get_user_from_db_by_email(conn: DbConnection, email_: String) -> Option<User> {
        block::<_, Option<(Email, User)>, _>(move || {
            use crate::schema::{emails::dsl::*, users::dsl::*};
            use diesel::prelude::*;
            emails
                .inner_join(users)
                .filter(email.eq(email_))
                .first(&conn)
                .optional()
        })
        .await
        .unwrap_or_else(|e| {
            error!("Could not query database: {}", e);
            None
        })
        .map(|(_, u)| u)
    }

    /// Check if an email exists in the telescope database.
    /// Will return false if the email doesn't exist or if there is an issue
    /// accessing the database.
    pub async fn email_registered(conn: DbConnection, email_: String) -> bool {
        block::<_, Option<Email>, _>(move || {
            use crate::schema::emails::dsl::*;
            use diesel::prelude::*;
            emails.find(email_).first(&conn).optional()
        })
        .await
        .unwrap_or_else(|e| {
            error!("Could not access emails in database: {}", e);
            None
        })
        .is_some()
    }

    /// Store an email in the database.
    pub async fn store(self) -> Result<(), TelescopeError> {
        // Get database connection
        let conn: DbConnection = AppData::global().get_db_conn().await?;

        // Asynchronously store this email record in the database.
        block::<_, Self, _>(move || {
            use crate::schema::emails::dsl::*;
            use diesel::prelude::*;
            diesel::insert_into(emails)
                .values(&self)
                .get_result(&conn)
        })
        .await
        .map_err(TelescopeError::from)
        .map(|stored| {
            trace!("Saved email to database: {:?}", stored);
        })
    }
}
