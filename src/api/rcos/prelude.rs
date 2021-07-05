//! Namespace types used by the RCOS API.

// Ignore compiler warnings for lowercase typenames.
#![allow(nonstandard_style)]

use crate::api::rcos::discord_assoications::ChannelType;
use crate::api::rcos::{
    meetings::MeetingType,
    users::{UserAccountType, UserRole},
};
use chrono::{DateTime, NaiveDate, Utc};
use url::Url;

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

/// List of strings for some reason not properly set in Hasura.
pub type _varchar = Vec<String>;

/// List of urls for some reason not properly set in Hasura.
pub type _url = Vec<Url>;

/// Discord channel association variants.
pub type channel_type = ChannelType;
