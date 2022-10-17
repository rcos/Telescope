//! API interactions for RCOS users from the central RCOS API.

pub mod accounts;
pub mod create;
pub mod delete;
pub mod developers_page;
pub mod discord_whois;
pub mod edit_profile;
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

    // Use serde rename, as this variant is spelled differently in the database and API.
    #[display(fmt = "Alum")]
    #[serde(rename = "alumn")]
    Alum,

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
    /// Constant array of all user roles.
    pub const ALL_ROLES: [UserRole; 7] = [
        UserRole::Student,
        UserRole::Alum,
        UserRole::Faculty,
        UserRole::FacultyAdvisor,
        UserRole::External,
        UserRole::ExternalMentor,
        UserRole::Sysadmin,
    ];

    /// Faculty advisors and sysadmins are considered admin users.
    pub fn is_admin(self) -> bool {
        self == UserRole::Sysadmin || self == UserRole::FacultyAdvisor
    }

    //TEMPORARY
    pub fn is_coordinator(self) -> bool {
        self == UserRole::Sysadmin || self == UserRole::FacultyAdvisor
    }

    /// Does this role represent an external user or mentor?
    pub fn is_external(self) -> bool {
        self == UserRole::External || self == UserRole::ExternalMentor
    }

    /// Can a user of role `a` change their role to role `b`?
    pub fn can_switch_to(a: Self, b: UserRole) -> bool {
        // Can always switch to the same role.
        if a == b {
            return true;
        }

        // Otherwise check the table.
        match (a, b) {
            // System administrators and faculty advisors can switch to anything.
            (UserRole::Sysadmin | UserRole::FacultyAdvisor, _) => true,

            // Faculty can switch to any non-admin role.
            (UserRole::Faculty, b) => !b.is_admin(),

            // Alumni can switch to student or external.
            (UserRole::Alum, UserRole::Student | UserRole::External) => true,

            // Students can switch to alum, or external user.
            (UserRole::Student, UserRole::Alum | UserRole::External) => true,

            // External users can become students (generally we should check for an RCS ID first).
            (UserRole::External, UserRole::Student) => true,

            // External mentors can drop their mentor status.
            (UserRole::ExternalMentor, UserRole::External) => true,

            // Any combination not explicitly listed above fails.
            _ => false,
        }
    }
}
