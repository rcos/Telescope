//! Calendar page and services

use crate::templates::{
    Template,
    calendar
};
use crate::error::TelescopeError;
use actix_web::web::{ServiceConfig, Json, Query};
use actix_web::HttpRequest;
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use chrono_tz::Tz;
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
        .service(calendar_page)
        .service(events);
}

/// Calendar page
#[get("/calendar")]
async fn calendar_page(req: HttpRequest) -> Result<Template, TelescopeError> {
    // Render calendar page.
    calendar::calendar_page(&req).await
}

/// Event endpoint query parameters used by FullCalendar.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct EventsQuery {
    /// The start time to get events from.
    pub start: NaiveDateTime,
    /// The end time to get events from.
    pub end: NaiveDateTime,
    /// The timezone of the
    #[serde(alias = "timeZone")]
    pub time_zone: Tz
}

/// Serializable event type used with FullCalendar.
/// See https://fullcalendar.io/docs/event-object.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullCalendarEvent {
    pub id: i64,
    pub title: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub url: Option<String>,
    /// The source object from the API.
    pub source: MeetingsMeetings
}

/// Events endpoint. This should return a JSON list of FullCalendarEvents.
#[get("/calendar/events")]
async fn events(identity: Identity, Query(params): Query<EventsQuery>) -> Result<Json<Vec<FullCalendarEvent>>, TelescopeError> {
    // Check if the user is authenticated -- can we show them events that aren't public.
    let is_authenticated: bool = identity.get_rcos_username().await?.is_some();

    // Convert timezones.
    let start_utc: DateTime<Utc> = params.time_zone
        // Get a timestamp with timezone
        .from_utc_datetime(&params.start)
        // Convert to UTC
        .with_timezone(&Utc);
    let end_utc: DateTime<Utc> = params.time_zone
        .from_utc_datetime(&params.end)
        .with_timezone(&Utc);

    // Return the meetings from the API.
    return Meetings::get(start_utc, end_utc, !is_authenticated)
        .await
        // Mapped into json.
        .map(|meetings| Json(meetings));
}
