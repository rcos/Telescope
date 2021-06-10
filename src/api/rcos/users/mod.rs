//! API interactions for RCOS users from the central RCOS API.

pub mod accounts;
pub mod create;
pub mod developers_page;
pub mod discord_whois;
pub mod enrollments;
pub mod navbar_auth;
pub mod profile;
pub mod role_lookup;

/// The valid user roles for all users in the RCOS database.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash, Display)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[display(fmt = "Student")]
    Student,
    #[display(fmt = "Faculty")]
    Faculty,
    #[display(fmt = "Faculty Advisor")]
    FacultyAdvisor,
    #[display(fmt = "Alumn")]
    Alumn,
    #[display(fmt = "External User")]
    External,
    #[display(fmt = "External Mentor")]
    ExternalMentor,
    #[display(fmt = "Telescope Admin")]
    Sysadmin,
}

/// The valid account types for all user accounts stored in the RCOS database.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Display, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum UserAccountType {
    #[display(fmt = "RCS")]
    Rpi,
    #[display(fmt = "Discord")]
    Discord,
    #[display(fmt = "Mattermost")]
    Mattermost,
    #[display(fmt = "GitHub")]
    GitHub,
    #[display(fmt = "GitLab")]
    GitLab,
    #[display(fmt = "BitBucket")]
    BitBucket,
}

impl UserRole {
    /// Faculty advisors and sysadmins are considered admin users.
    pub fn is_admin(self) -> bool {
        self == UserRole::Sysadmin || self == UserRole::FacultyAdvisor
    }
}
