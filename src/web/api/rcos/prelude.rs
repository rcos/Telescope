//! Namespace types used by the RCOS API.

// Ignore compiler warnings for lowercase typenames.
#![allow(nonstandard_style)]

use chrono::{Utc, DateTime, NaiveDate};
use crate::web::api::rcos::{
    users::{
        UserAccountType,
        UserRole
    },
    meetings::MeetingType
};

/// Timestamp with Timezone.
pub type timestamptz = DateTime<Utc>;

/// Date (the ones in the database do not have a timezone,
/// but should be interpreted as eastern time).
pub type date = NaiveDate;

/// User's role.
pub type user_role = UserRole;

/// User account variants.
pub type user_account = UserAccountType;

/// Meeting variants.
pub type meeting_type = MeetingType;

/// List of strings for some reason not properly set in GraphQL.
pub type _varchar = Vec<String>;
