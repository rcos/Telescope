use crate::{
    models::User,
    schema::emails,
    web::{
        DbConnection,
        api::graphql::ApiContext
    }
};
use juniper::{FieldError, FieldResult, Value};
use regex::Regex;
use uuid::Uuid;
use actix_web::web::block;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r#"^[[:alpha:]]+@[[:alpha:]]+(\.[[:alpha:]]+)+$"#
    ).unwrap();
}

/// Field structure must match that in the SQL migration.
/// (for diesel reasons it seems)
#[derive(Clone, Serialize, Deserialize, Insertable, Queryable, Debug)]
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
#[juniper::object(Context = ApiContext)]
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
    /// Get the email validation regex.
    pub fn get_email_regex() -> &'static Regex {
        &*EMAIL_REGEX
    }

    /// Create a new email object. Return none if email does not
    /// match regex.
    pub fn new<T: Into<String>>(user_id: Uuid, email: T) -> Option<Self> {
        let email = email.into();
        if Self::get_email_regex().is_match(&email) {
            Some(Self {
                user_id,
                email,
                is_visible: false,
            })
        } else {
            None
        }
    }

    /// Try to get a user based on an email from the database.
    pub async fn get_user_from_db_by_email(conn: DbConnection, email_: String) -> Option<User> {
        block::<_, (Email, User), _>(move || {
            use crate::schema::{
                users::dsl::*,
                emails::dsl::*
            };
            use diesel::prelude::*;
            emails.inner_join(users)
                .filter(email.eq(email_))
                .first(&conn)
        }).await
            .ok()
            .map(|(_, u)| u)
    }
}
