//! Queries and mutations to the RCOS API for meeting data.

pub mod public;

/// The type of a meeting.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingType {
    LargeGroup,
    SmallGroup,
    Presentations,
    BonusSession,
    Grading,
    Mentors,
    Coordinators,
    Other,
}
