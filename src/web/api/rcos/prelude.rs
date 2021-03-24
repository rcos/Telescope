//! Namespace types used by the RCOS API.

// Ignore compiler warnings for lowercase typenames.
#![allow(nonstandard_style)]

use chrono::{Utc, DateTime};
use crate::web::api::rcos::{
    users::{
        UserAccountType,
        UserRole
    },
    meetings::MeetingType
};

/// Timestamp with Timezone.
pub type timestamptz = DateTime<Utc>;

/// User's role.
pub type user_role = UserRole;

/// User account variants.
pub type user_account = UserAccountType;

/// Meeting variants.
pub type meeting_type = MeetingType;
