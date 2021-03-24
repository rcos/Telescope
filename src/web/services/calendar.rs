//! Calendar page and services

use crate::templates::{
    Template,
    calendar
};
use crate::error::TelescopeError;
use actix_web::web::{ServiceConfig, Json, Query};
use actix_web::HttpRequest;
use chrono::{DateTime, Utc};
use crate::web::services::auth::identity::Identity;

/// Register calendar related services.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(calendar_page);
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
    pub start: DateTime<Utc>,
    /// The end time to get events from.
    pub end: DateTime<Utc>
}

/// Serializable event type used with FullCalendar.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct FullCalendarEvent {

}

/// Events endpoint. This should return a JSON list of FullCalendarEvents.
#[get("/calendar/events")]
async fn events(identity: Identity, Query(params): Query<EventsQuery>) -> Result<Json<Vec<FullCalendarEvent>>, TelescopeError> {
    // Check if the user is authenticated -- can we show them events that aren't public.
    let is_authenticated: bool = identity.get_rcos_username().await?.is_some();

    Err(TelescopeError::NotImplemented)
}
