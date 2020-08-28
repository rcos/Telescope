use crate::schema::emails;
use uuid::Uuid;
use regex::Regex;

lazy_static!{
    static ref EMAIL_REGEX: Regex = {
        Regex::new(r#"^[[:alpha:]]+@[[:alpha:]]+(\.[[:alpha:]]+)+$"#).unwrap()
    };
}

#[derive(Clone, Serialize, Deserialize, Insertable, Queryable, Debug, juniper::GraphQLObject)]
#[table_name = "emails"]
#[graphql(description = "An email of an RCOS user.")]
pub struct Email {
    /// User id of associated user
    pub user_id: Uuid,
    /// is this email displayed on the website publicly?
    pub is_visible: bool,
    /// The email
    pub email: String,
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
