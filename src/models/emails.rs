use crate::schema::emails;
use uuid::Uuid;
use regex::Regex;

lazy_static!{
    static ref EMAIL_REGEX: Regex = {
        Regex::new(r#"^[[:alpha:]]+@[[:alpha:]]+(\.[[:alpha:]]+)+$"#).unwrap()
    };
}

/// Field structure must match that in the SQL migration.
/// (for diesel reasons it seems)
#[derive(Clone, Serialize, Deserialize, Insertable, Queryable, Debug, juniper::GraphQLObject)]
#[table_name = "emails"]
#[graphql(description = "An email of an RCOS user.")]
pub struct Email {
    /// The email
    pub email: String,
    /// is this email displayed on the website publicly?
    #[graphql(skip)]
    pub is_visible: bool,
    /// User id of associated user
    pub user_id: Uuid,
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
                is_visible: false
            })
        } else {
            None
        }

    }
}
