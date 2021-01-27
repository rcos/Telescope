use uuid::Uuid;
use crate::schema::attendance_records;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Insertable, Associations)]
#[table_name = "attendance_records"]
pub struct AttendanceRecord {
    /// The id of the user attending the event.
    uid: Uuid,
    /// The id of the event they were attending.
    aid: Uuid,
    /// Were they there?
    present: bool
}