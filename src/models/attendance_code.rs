use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::schema::attendance_codes;

#[derive(Clone, Deserialize, Serialize, Debug, Insertable, Queryable, Associations)]
#[table_name = "attendance_codes"]
pub struct AttendanceCode {
    /// The attendance/event ID.
    id: Uuid,
    /// The id of the user who created it.
    creator_id: Uuid,
    /// Timestamp that the attendance code was created at.
    created_at: DateTime<Utc>,
    /// Timestamp that the attendance code expires at.
    expires_at: DateTime<Utc>,
    /// The code itself. This is None if not used.
    code: Option<String>,
    /// Can attendance be taken using the link?
    enable_link: bool,
    /// The passphrase to get attendance. This is none if not used.
    phrase: Option<String>,
}
