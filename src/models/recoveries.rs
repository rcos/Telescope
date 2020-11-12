use crate::{
    models::User,
    schema::lost_passwords
};

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Queryable, Insertable, Serialize, Deserialize, Associations)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "lost_passwords"]
pub struct Recovery {
    /// The recovery ID
    recovery_id: Uuid,
    /// The user ID
    user_id: Uuid,
    /// When this recovery expires.
    expiration: DateTime<Utc>,
}

impl Recovery {
    /// Users have 10 minutes to recover their passwords before the recovery goes stale.
    fn get_expiration_duration() -> Duration {
        Duration::minutes(10)
    }

    /// Get the expiration time by adding the expiration duration to the current time.
    fn get_expiration_from_now() -> DateTime<Utc> {
        Utc::now() + Self::get_expiration_duration()
    }
}
