//! Models pertaining to users and their accounts on the RCOS database.

use chrono::{DateTime, Utc};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub preferred_name: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub cohort: Option<i32>,
    pub role: UserRole,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct UserAccount {
    username: String,
    #[serde(rename = "type")]
    ty: UserAccountType,
    account_id: String,
    created_at: DateTime<Utc>,
}
