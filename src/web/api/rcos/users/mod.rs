//! API interactions for RCOS users from the central RCOS API.

pub mod accounts;
pub mod create;
pub mod developers_page;

/// The valid user roles for all users in the RCOS database.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Student,
    Faculty,
    FacultyAdvisor,
    Alumn,
    External,
    ExternalMentor,
}

/// The valid account types for all user accounts stored in the RCOS database.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserAccountType {
    Rpi,
    Discord,
    Mattermost,
    GitHub,
    GitLab,
    BitBucket,
}
