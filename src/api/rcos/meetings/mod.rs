//! Queries and mutations to the RCOS API for meeting data.

pub mod authorization_for;
pub mod creation;
pub mod get;
pub mod get_by_id;
pub mod get_host;
pub mod edit;

/// List of all existing meeting type variants.
pub const ALL_MEETING_TYPES: [MeetingType; 8] = [
    MeetingType::LargeGroup,
    MeetingType::SmallGroup,
    MeetingType::Presentations,
    MeetingType::BonusSession,
    MeetingType::Grading,
    MeetingType::Mentors,
    MeetingType::Coordinators,
    MeetingType::Other,
];

/// The type of a meeting.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Display)]
#[serde(rename_all = "snake_case")]
pub enum MeetingType {
    #[display(fmt = "Large Group")]
    LargeGroup,
    #[display(fmt = "Small Group")]
    SmallGroup,
    #[display(fmt = "Presentation")]
    Presentations,
    #[display(fmt = "Bonus Session")]
    BonusSession,
    #[display(fmt = "Grading Meeting")]
    Grading,
    #[display(fmt = "Mentor Meeting")]
    Mentors,
    #[display(fmt = "Coordinator Meeting")]
    Coordinators,
    #[display(fmt = "Uncategorized Meeting")]
    Other,
}
