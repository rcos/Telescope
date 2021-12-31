//! List of meetings page.

use crate::api::rcos::meetings::authorization_for::{AuthorizationFor, UserMeetingAuthorization};
use crate::api::rcos::meetings::get::Meetings;
use crate::api::rcos::meetings::MeetingType;
use crate::error::TelescopeError;
use crate::templates::page::Page;
use crate::templates::Template;
use crate::web::services::auth::identity::Identity;
use actix_web::web::{Query, ServiceConfig};
use actix_web::HttpRequest;
use chrono::{Duration, Local, NaiveDate, TimeZone, Utc};
use crate::api::rcos::meetings::{
    self,
    get::meetings::{MeetingsMeetings}
};

/// Register the meetings page.
pub fn register(c: &mut ServiceConfig) -> &mut ServiceConfig {
    c.service(meetings_list)
}

/// The path to the template's handlebars file.
const TEMPLATE_PATH: &'static str = "meetings/list";

/// Query parameters submitted via the form on the meetings page.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum MeetingsQuery {
    /// All meetings between a start and end date.
    BetweenDates {
        /// The start time to get events from.
        start: NaiveDate,
        /// The end time to get events from.
        end: NaiveDate,
    },

    /// All meetings in a given semester.
    Semester {
        /// The specified semester.
        semester_id: String
    }
}

/// Meetings page
#[get("/meetings")]
async fn meetings_list(
    req: HttpRequest,
    params: Option<Query<MeetingsQuery>>,
    identity: Identity,
) -> Result<Page, TelescopeError> {
    // Is there an RCOS user authenticated?
    let viewer: Option<_> = identity.get_user_id().await?;
    // Check if that user can view drafts / certain meeting types.
    let authorization: UserMeetingAuthorization = AuthorizationFor::get(viewer).await?;
    let include_drafts: bool = authorization.can_view_drafts();
    let visible_meeting_types: Vec<MeetingType> = authorization.viewable_types();
    // Create the base filter that will only accept the right meeting types and set draft visibility.
    let base_filter: meetings::get::meetings::Variables = meetings::get::meetings::Variables {
        accept_types: visible_meeting_types,
        include_drafts,
        // Default remaining fields for now.
        ..Default::default()
    };

    // Remove the query structure from the middle of the parameters.
    let params: Option<MeetingsQuery> = params.map(|p| p.0);

    // Get the values to pre-fill in the filters if possible
    let query = params
        // Clone to avoid use-after-move.
        .clone()
        // Otherwise convert the API parameters
        .unwrap_or(MeetingsQuery::BetweenDates {
            start: Local::today().naive_local(),
            end: (Local::today() + Duration::weeks(1)).naive_local(),
        });

    // Make an API call. Determine the filter based on the query in the URL.
    let meetings: Vec<MeetingsMeetings> = match params {
        // All the meetings between two dates.
        Some(MeetingsQuery::BetweenDates {start, end}) => {
            // Convert dates to UTC.
            let start = Local.from_local_date(&start)
                // Take the earlier time if ambiguous.
                .earliest()
                // Add time to make timestamp.
                .map(|date| date.and_hms(0, 0, 0))
                // Default to recently if this fails.
                .unwrap_or(Local::now() - Duration::hours(2))
                // Convert to UTC.
                .with_timezone(&Utc);

            let end = Local.from_local_date(&end)
                // Take the latest time if ambiguous.
                .latest()
                // Add time to make timestamp.
                .map(|date| date.and_hms(0, 0, 0))
                // Default to a week from now if this fails.
                .unwrap_or(Local::now() + Duration::weeks(1))
                // Convert to UTC.
                .with_timezone(&Utc);

            // Call API.
            Meetings::get(meetings::get::meetings::Variables {
                time_filter: Some(meetings::get::meetings::timestamptz_comparison_exp {
                    _gt: Some(start),
                    _lt: Some(end),
                    // Have to set this to false instead of null to satisfy hasura.
                    _is_null: Some(false),
                    // Default all other fields.
                    ..Default::default()
                }),
                // Default other variables.
                ..base_filter
            }).await?
        },

        // All the meetings for a given semester.
        Some(MeetingsQuery::Semester { semester_id }) => {
            Meetings::get(meetings::get::meetings::Variables {
                semester_id_filter: Some(meetings::get::meetings::String_comparison_exp {
                    _eq: Some(semester_id),
                    ..Default::default()
                }),
                ..base_filter
            }).await?
        },

        // No query supplied.
        None => {
            // By default, just get the meetings for the next week or so.
            Meetings::get(meetings::get::meetings::Variables {
                time_filter: Some(meetings::get::meetings::timestamptz_comparison_exp {
                    _gt: Some(Utc::now() - Duration::hours(2)),
                    _lt: Some(Utc::now() + Duration::weeks(1)),
                    // Have to set this to false instead of null to satisfy hasura.
                    _is_null: Some(false),
                    // Default all other filter fields.
                    ..Default::default()
                }),
                // Default other variable fields.
                ..base_filter
            }).await?
        },
    };

    let mut template = Template::new(TEMPLATE_PATH);
    template.fields = json!({
        "meetings": meetings,
        "query": query,
        "authorization": authorization,
    });

    return template.in_page(&req, "RCOS Meetings").await;
}
