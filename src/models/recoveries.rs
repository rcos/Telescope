use crate::{models::users::User, schema::lost_passwords};

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;
use crate::web::DbConnection;
use actix_web::web::block;
use diesel::RunQueryDsl;
use crate::util::handle_blocking_err;

#[derive(Clone, Debug, Queryable, Insertable, Serialize, Deserialize, Associations)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "lost_passwords"]
pub struct Recovery {
    /// The recovery ID
    recovery_id: Uuid,
    /// The user ID
    user_id: Uuid,
    /// When this recovery expires.
    pub expiration: DateTime<Utc>,
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

    /// Create a new password recovery.
    pub fn for_user(user: &User) -> Self {
        Self {
            recovery_id: Uuid::new_v4(),
            user_id: user.id,
            expiration: Self::get_expiration_from_now()
        }
    }

    /// Store a recovery in the database. On success, return the saved recovery.
    pub async fn store(self, db_conn: DbConnection) -> Result<Self, String> {
        block::<_, Self, _>(move || {
            use diesel::prelude::*;
            use crate::schema::lost_passwords::dsl::*;
            diesel::insert_into(lost_passwords)
                .values(&self)
                .get_result(&db_conn)
        })
            .await
            .map(|r| {
                trace!("Saved 1 password recovery to database: {:?}", r);
                r
            })
            .map_err(|e| handle_blocking_err(e, "Could not access database."))
    }
}
