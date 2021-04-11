//! Queries and mutations to the RCOS API for meeting data.

pub mod get;
pub mod get_by_id;

/// The type of a meeting.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Display)]
#[serde(rename_all = "snake_case")]
pub enum MeetingType {
    #[display(fmt = "Large Group")]
    LargeGroup,
    #[display(fmt = "Small Group")]
    SmallGroup,
    #[display(fmt = "Presentations")]
    Presentations,
    #[display(fmt = "Bonus Session")]
    BonusSession,
    #[display(fmt = "Grading")]
    Grading,
    #[display(fmt = "Mentor Meeting")]
    Mentors,
    #[display(fmt = "Coordinator Meeting")]
    Coordinators,
    #[display(fmt = "Uncategorized Meeting")]
    Other,
}
