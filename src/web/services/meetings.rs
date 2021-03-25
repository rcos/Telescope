//! Calendar page and services

use crate::templates::{
    Template,
    meetings
};
use crate::error::TelescopeError;
use actix_web::web::{ServiceConfig, Query};
use actix_web::HttpRequest;
use chrono::{DateTime, Utc, TimeZone, Local, Duration, NaiveDate, Date};
use crate::web::services::auth::identity::Identity;
use crate::web::api::rcos::meetings::get::{
    Meetings,
    meetings::{
        MeetingsMeetings
    }
};

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(calendar_page);
}

/// Event endpoint query parameters used by FullCalendar.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct MeetingsQuery {
    /// The start time to get events from.
    pub start: Option<NaiveDate>,
    /// The end time to get events from.
    pub end: Option<NaiveDate>,
}

/// Calendar page
#[get("/meetings")]
async fn calendar_page(req: HttpRequest, params: Option<Query<MeetingsQuery>>, identity: Identity) -> Result<Template, TelescopeError> {
    // Resolve parameters to API query variables
    let start: DateTime<Utc> = params.as_ref()
        // Extract the start parameter from the query
        .and_then(|p| p.start.as_ref())
        // Convert to a date in the local timezone
        .map(|naive: &NaiveDate| Local.from_local_date(naive))
        // If it's ambiguous what date to use in the local timezone, pick the earlier one.
        .and_then(|local_result| local_result.earliest())
        // Conver the date to a timestamp of the beginning of the day
        .map(|date: Date<Local>| date.and_hms(0,0,0))
        // If there is no valid timezone or the start parameter wasn't supplied,
        // use the current time minus 2 hours. This should be sufficient to catch all
        // recent and ongoing meetings.
        .unwrap_or(Local::now() - Duration::hours(2))
        // Convert timezone to UTC.
        .with_timezone(&Utc);

    let end: DateTime<Utc> = params.as_ref()
        // Extract the end parameter from the query
        .and_then(|p| p.end.as_ref())
        // Convert to a date in the local timezone.
        .map(|naive: &NaiveDate| Local.from_local_date(naive))
        // If the date in the local timezone is ambiguous, use the later one
        .and_then(|local_result| local_result.latest())
        // Convert the date to a timestamp at midnight.
        .map(|date: Date<Local>| date.and_hms(0,0,0))
        // If there is no valid time, or the parameter wasn't supplied,
        // default to one week from today. This will show all the next meetings.
        .unwrap_or(Local::now() + Duration::weeks(1))
        // Convert timezone to UTC.
        .with_timezone(&Utc);

    // Is there an RCOS user authenticated?
    let is_authenticated: bool = identity.get_rcos_username().await?.is_some();
    // The visibility of private events is true if there's an authenticated user.
    let public_only: bool = !is_authenticated;

    // Query the RCOS API to get meeting data.
    let events: Vec<MeetingsMeetings> = Meetings::get(start, end, public_only).await?;
    // Build a meetings page template, render it into a page for the user.
    return meetings::make(events, params.map(|q| q.0))
        .render_into_page(&req, "RCOS Meetings")
        .await;
}
