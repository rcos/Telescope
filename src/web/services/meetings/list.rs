//! List of meetings page.

use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::api::rcos::meetings::get::GetMeetings;
use crate::api::rcos::meetings::MeetingType;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::{Query, ServiceConfig};
use actix_web::HttpRequest;
use chrono::{Date, DateTime, Duration, Local, NaiveDate, TimeZone, Utc};

/// Register the meetings page.
pub fn register(c: &mut ServiceConfig) -> &mut ServiceConfig {
    c.service(meetings_list)
}

/// The path to the template's handlebars file.
const TEMPLATE_PATH: &'static str = "meetings/list";

/// Query parameters submitted via the form on the meetings page.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct MeetingsQuery {
    /// The start time to get events from.
    pub start: NaiveDate,
    /// The end time to get events from.
    pub end: NaiveDate,
}

/// Meetings page
#[get("/meetings")]
async fn meetings_list(
    req: HttpRequest,
    params: Option<Query<MeetingsQuery>>,
    identity: Identity,
) -> Result<Page, TelescopeError> {
    // Resolve parameters to API query variables
    let start: DateTime<Utc> = params
        .as_ref()
        // Extract the start parameter from the query
        .map(|p| p.start)
        // Convert to a date in the local timezone
        .map(|naive: NaiveDate| Local.from_local_date(&naive))
        // If it's ambiguous what date to use in the local timezone, pick the earlier one.
        .and_then(|local_result| local_result.earliest())
        // Conver the date to a timestamp of the beginning of the day
        .map(|date: Date<Local>| date.and_hms(0, 0, 0))
        // If there is no valid timezone or the start parameter wasn't supplied,
        // use the current time minus 2 hours. This should be sufficient to catch all
        // recent and ongoing meetings.
        .unwrap_or(Local::now() - Duration::hours(2))
        // Convert timezone to UTC.
        .with_timezone(&Utc);

    let end: DateTime<Utc> = params
        .as_ref()
        // Extract the end parameter from the query
        .map(|p| p.end)
        // Convert to a date in the local timezone.
        .map(|naive: NaiveDate| Local.from_local_date(&naive))
        // If the date in the local timezone is ambiguous, use the later one
        .and_then(|local_result| local_result.latest())
        // Convert the date to a timestamp near midnight that night.
        .map(|date: Date<Local>| date.and_hms(23, 59, 59))
        // If there is no valid time, or the parameter wasn't supplied,
        // default to one week from today. This will show all the next meetings.
        .unwrap_or(Local::now() + Duration::weeks(1))
        // Convert timezone to UTC.
        .with_timezone(&Utc);

    // Is there an RCOS user authenticated?
    let viewer: Option<_> = identity.get_user_id().await?;
    // Check if that user can view drafts / certain meeting types.
    let authorization: UserMeetingAuthorization = AuthorizationFor::get(viewer).await?;
    let include_drafts: bool = authorization.can_view_drafts();
    let visible_meeting_types: Vec<MeetingType> = authorization.viewable_types();

    // Query the RCOS API to get meeting data.
    let events: Vec<_> = GetMeetings::execute(start, end, include_drafts, visible_meeting_types).await?;

    // Get the values to pre-fill in the filters.
    let query = params
        // The existing query if there was one
        .map(|p| p.0)
        // Otherwise convert the API parameters
        .unwrap_or(MeetingsQuery {
            start: start.naive_local().date(),
            end: end.naive_local().date(),
        });

    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "meetings": events,
        "query": query,
        "authorization": authorization,
    });

    return template.in_page(&req, "RCOS Meetings").await;
}
