use crate::schema::confirmations;
use chrono::{DateTime, Duration, Local};
use uuid::Uuid;

/// An email to a user asking them to confirm their email (and possibly set up an account).
#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "confirmations"]
pub struct Confirmation {
    invite_id: Uuid,
    email: String,
    user_id: Option<Uuid>,
    expiration: DateTime<Local>,
}

impl Confirmation {
    /// Currently invites expire after 3 days.
    fn get_expiration_duration() -> Duration {
        Duration::days(3)
    }

    /// Get the current datetime and add tge expiration time.
    fn get_expiration_time_from_now() -> DateTime<Local> {
        Local::now() + Self::get_expiration_duration()
    }

    /// Does this invite create a new user, requiring the creation of a password
    pub fn creates_user(&self) -> bool {
        self.user_id.is_none()
    }
}
