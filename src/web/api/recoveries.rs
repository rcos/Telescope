
use crate::schema::lost_passwords;

use uuid::Uuid;
use chrono::{DateTime, Local, Duration};

#[derive(Clone, Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "lost_passwords"]
pub struct Recovery {
    /// The recovery ID
    recovery_id: Uuid,
    /// The user ID
    user_id: Uuid,
    /// When this recovery expires.
    expiration: DateTime<Local>
}

impl Recovery {
    /// Users have 6 hours to recover their passwords before the recovery goes stale.
    fn get_expiration_duration() -> Duration {
        Duration::hours(6)
    }

    /// Get the expiration time by adding the expiration duration to the current time.
    fn get_expiration_from_now() -> DateTime<Local> {
        Local::now() + Self::get_expiration_duration()
    }


}