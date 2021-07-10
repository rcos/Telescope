//! Services to support meeting edits.

use actix_web::{web::{ServiceConfig, Path, Query}, HttpResponse, HttpRequest};
use crate::error::TelescopeError;
use crate::web::services::auth::identity::AuthenticationCookie;
use crate::api::rcos::meetings::{
    authorization_for::{UserMeetingAuthorization, AuthorizationFor},
    creation::context::get_context,
    get_by_id::{
        Meeting,
        meeting::MeetingMeeting
    },
    edit,
};
use crate::templates::forms::FormTemplate;
use chrono::{DateTime, Utc, Local, NaiveTime, NaiveDateTime, TimeZone};
use crate::api::rcos::meetings::ALL_MEETING_TYPES;
use serde_json::Value;
use crate::web::services::meetings::create::{FinishForm, get_semester_bounds};
use actix_web::web::Form;
use crate::api::rcos::meetings::creation::create::normalize_url;
use actix_web::http::header::LOCATION;
use crate::templates::Template;
use crate::api::rcos::meetings::edit::EditHostSelection;

/// The Handlebars file for the meeting edit form.
const MEETING_EDIT_FORM: &'static str = "meetings/edit/form";

/// The Handlebars file for the host selection page.
const HOST_SELECTION_TEMPLATE: &'static str = "meetings/edit/host_selection";

/// Register the meeting edit services.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(edit_page)
        .service(submit_meeting_edits)
        .service(host_selection);
}

/// Structure for query which can optionally be passed to the edit page to set a new host.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct HostQuery {
    /// The new host for the meeting. Empty string for no host.
    set_host: String
}

/// Get meeting data or return a resource not found error.
async fn get_meeting_data(meeting_id: i64) -> Result<MeetingMeeting, TelescopeError> {
    // Get the meeting data to check that it exists.
    Meeting::get_by_id(meeting_id)
        .await?
        .ok_or(TelescopeError::resource_not_found("Meeting Not Found", "Could not find a meeting for this ID."))
}

/// Get a user's meeting authorization object from their authentication cookie.
async fn authorization_for_viewer(auth: &AuthenticationCookie) -> Result<UserMeetingAuthorization, TelescopeError> {
    // Get username from cookie.
    let viewer: String = auth.get_rcos_username_or_error().await?;

    // Query API for auth object.
    return AuthorizationFor::get(Some(viewer)).await;
}

/// Get meeting data and error if the authenticated user cannot edit the meeting.
async fn meeting_data_checked(auth: &AuthenticationCookie, meeting_id: i64) -> Result<MeetingMeeting, TelescopeError> {
    // Get meeting data. Extract host's username.
    let meeting_data = get_meeting_data(meeting_id).await?;
    let meeting_host: Option<&str> = meeting_data.host.as_ref().map(|host| host.username.as_str());

    // Get user's authorization object.
    let authorization = authorization_for_viewer(auth).await?;

    // Check edit access.
    if !authorization.can_edit(meeting_host) {
        return Err(TelescopeError::Forbidden);
    } else {
        return Ok(meeting_data);
    }
}

/// Resolve the desired host username from the set host query parameter or the existing meeting
/// host.
fn resolve_host_username(meeting_data: &MeetingMeeting, set_host: Option<Query<HostQuery>>) -> Option<String> {
    let existing_host: Option<String> = meeting_data.host.as_ref().map(|h| h.username.clone());
    let new_host: Option<String> = set_host.map(|q| q.0.set_host);
    return new_host
        .or(existing_host)
        // Require that host string is not empty. If it is, no host.
        .filter(|host| !host.trim().is_empty());
}

/// Resolve the meeting title value. This is the supplied title or a combination of the meeting
/// type and date.
fn resolve_meeting_title(meeting_data: &MeetingMeeting) -> String {
    match meeting_data.title.as_ref() {
        Some(title) => title.clone(),
        None => {
            let meeting_start: &DateTime<Utc> = &meeting_data.start_date_time;
            format!("{} - {}", meeting_data.type_, meeting_start.with_timezone(&Local).format("%B %_d, %Y"))
        }
    }
}

/// Create the form template for meeting edits.
fn make_form(meeting_data: &MeetingMeeting) -> FormTemplate {
    // Resolve the meeting title.
    let meeting_title: String = resolve_meeting_title(&meeting_data);
    // Create the template.
    return FormTemplate::new(MEETING_EDIT_FORM, format!("Edit {}", meeting_title));
}

/// Service to display meeting edit form to users who can edit the meeting.
#[get("/meeting/{meeting_id}/edit")]
async fn edit_page(
    Path(meeting_id): Path<i64>,
    auth: AuthenticationCookie,
    set_host: Option<Query<HostQuery>>
) -> Result<FormTemplate, TelescopeError> {
    // Get the meeting data. Error on meeting not found or permissions failure.
    let meeting_data = meeting_data_checked(&auth, meeting_id).await?;
    // Resolve the desired host username.
    let host: Option<String> = resolve_host_username(&meeting_data, set_host);
    // Get the creation context (based on the resolved host)
    // so we know what semesters are available.
    let context: Value = get_context(host).await?;

    // Create the meeting template.
    let mut form: FormTemplate = make_form(&meeting_data);
    // Instantiate form with meeting data, context, and meeting types.
    form.template = json!({
        "data": &meeting_data,
        "meeting_types": ALL_MEETING_TYPES,
        "context": context
    });

    // Add fields to the template converting the timestamps in the meeting data to the HTML versions.
    let meeting_start: &DateTime<Utc> = &meeting_data.start_date_time;
    let meeting_start_local: DateTime<Local> = meeting_start.with_timezone(&Local);
    form.template["data"]["start_date"] = json!(meeting_start_local.format("%Y-%m-%d").to_string());
    form.template["data"]["start_time"] = json!(meeting_start_local.format("%H:%M").to_string());

    let meeting_end: &DateTime<Utc> = &meeting_data.end_date_time;
    let meeting_end_local: DateTime<Local> = meeting_end.with_timezone(&Local);
    form.template["data"]["end_date"] = json!(meeting_end_local.format("%Y-%m-%d").to_string());
    form.template["data"]["end_time"] = json!(meeting_end_local.format("%H:%M").to_string());

    return Ok(form);
}

#[post("/meeting/{meeting_id}/edit")]
async fn submit_meeting_edits(
    Path(meeting_id): Path<i64>,
    auth: AuthenticationCookie,
    set_host: Option<Query<HostQuery>>,
    // Use the same structure as is used for creation since the
    // form data submitted should be the same.
    Form(form_data): Form<FinishForm>,
) -> Result<HttpResponse, TelescopeError> {
    // Get meeting data. Error if there is no such meeting or the user cannot access it
    let meeting_data = meeting_data_checked(&auth, meeting_id).await?;
    // Resolve the desired host username.
    let host: Option<String> = resolve_host_username(&meeting_data, set_host);
    // Get the creation context (based on the resolved host)
    // so we know what semesters are available.
    let context: Value = get_context(host).await?;

    // Create the meeting template.
    let mut form: FormTemplate = make_form(&meeting_data);
    // Instantiate form with meeting types, context and data.
    form.template = json!({
        "meeting_types": ALL_MEETING_TYPES,
        "context": &context,
        "data": &meeting_data
    });

    // Destructure the submitted form.
    let FinishForm {
        start_time,
        start_date,
        end_time,
        end_date,
        description,
        external_slides_url,
        is_remote,
        is_draft,
        semester,
        recording_url,
        meeting_url,
        location,
        kind,
        title
    } = form_data;

    // Like the creation system, semester ID, meeting kind, and host username are not validated.

    // Add submitted data to return form.
    form.template["data"]["semester"] = json!({"semester_id": &semester});
    form.template["data"]["type"] = json!(kind);
    form.template["data"]["description"] = json!(&description);

    form.template["data"]["start_date"] = json!(&start_date);
    form.template["data"]["end_date"] = json!(&end_date);
    form.template["data"]["start_time"] = json!(&start_time);
    form.template["data"]["end_time"] = json!(&end_time);

    // Handle meeting title -- just whitespace and default to None if empty.
    let title: Option<String> = (!title.trim().is_empty()).then(|| title.trim().to_string());
    form.template["data"]["title"] = json!(&title);

    // Same with location.
    let location: Option<String> = location
        .and_then(|string| (!string.trim().is_empty()).then(|| string.trim().to_string()));
    form.template["data"]["location"] = json!(&location);

    // Trim description.
    let description: String = description.trim().to_string();
    form.template["data"]["description"] = json!(&description);

    // Don't bother trimming URLs, since the GraphQL mutation will normalize them.
    form.template["data"]["meeting_url"] = json!(&meeting_url);
    form.template["data"]["recording_url"] = json!(&recording_url);
    form.template["data"]["external_slides_url"] = json!(&external_slides_url);

    // Handle flags.
    let is_remote: bool = is_remote.unwrap_or(false);
    let is_draft: bool = is_draft.unwrap_or(false);
    form.template["data"]["is_remote"] = json!(is_remote);
    form.template["data"]["is_draft"] = json!(is_draft);

    // Validate dates and set an issue in the form if there is one.
    // Get the selected semester info from the context object.
    let selected_semester: &Value = form.template["context"]["available_semesters"]
        .as_array()
        .expect("There should be an available semesters array in the meeting context.")
        .iter()
        .find(|available_semester| available_semester["semester_id"] == semester.as_str())
        .ok_or(TelescopeError::BadRequest {
            header: "Malformed Meeting Edit Form".into(),
            message: "Selected semester in available semester list.".into(),
            show_status_code: false
        })?;

    // Get the semester bounds.
    let (semester_start, semester_end) = get_semester_bounds(selected_semester);

    if end_date < start_date {
        form.template["issues"]["end_date"] = json!("End date is before start date.");
    }
    else if start_date > semester_end {
        form.template["issues"]["start_date"] = json!("Start date is after end of semester.");
    }
    else if end_date > semester_end {
        form.template["issues"]["end_date"] = json!("End date is after end of semester.");
    }
    else if start_date < semester_start {
        form.template["issues"]["start_date"] = json!("Start date is before semester starts.");
    }
    else if end_date < semester_start {
        form.template["issues"]["end_date"] = json!("End date is before semester starts.");
    }

    // If there was an issue, return the form as invalid.
    if form.template["issues"] != json!(null) {
        return Err(TelescopeError::invalid_form(&form));
    }

    // Parse times
    let time_parse = |time: String| format!("{}:00", time).parse::<NaiveTime>();

    let start_time: NaiveTime = time_parse(start_time).map_err(|e| TelescopeError::BadRequest {
        header: "Malformed Start Time".into(),
        message: format!("Could not parse start time. Internal error: {}", e),
        show_status_code: false
    })?;

    let end_time: NaiveTime = time_parse(end_time).map_err(|e| TelescopeError::BadRequest {
        header: "Malformed End Time".into(),
        message: format!("Could not parse end time. Internal error: {}", e),
        show_status_code: false
    })?;

    // Add times to dates.
    let start: NaiveDateTime = start_date.and_time(start_time);
    let end: NaiveDateTime = end_date.and_time(end_time);

    // Make sure meeting starts before it ends.
    if start > end {
        form.template["issues"]["end_time"] = json!("End time is before start time.");
        return Err(TelescopeError::invalid_form(&form));
    }

    // Add timestamps.
    let timezone_adder = |timestamp: &NaiveDateTime| Local
        .from_local_datetime(timestamp)
        .single();

    let start: DateTime<Local> = timezone_adder(&start)
        .ok_or(TelescopeError::BadRequest {
            header: "Malformed Start Time".into(),
            message: "Could not ascribe local timezone to start timestamp.".into(),
            show_status_code: false
        })?;

    let end: DateTime<Local> = timezone_adder(&end)
        .ok_or(TelescopeError::BadRequest {
            header: "Malformed End Time".into(),
            message: "Could not ascribe local timezone to end timestamp.".into(),
            show_status_code: false,
        })?;

    // Create variables for mutation.
    let edit_mutation_variables = edit::edit_meeting::Variables {
        meeting_id,
        title,
        start: start.with_timezone(&Utc),
        end: end.with_timezone(&Utc),
        semester_id: semester,
        kind,
        description,
        is_remote,
        is_draft,
        meeting_url: normalize_url(meeting_url),
        location,
        external_slides_url: normalize_url(external_slides_url),
        recording_url: normalize_url(recording_url),
        // Extract the host from context object.
        host: form.template["context"]
            .get("host")
            .and_then(|host| host[0]["username"].as_str())
            .map(|host| host.to_string())
    };

    // The returned meeting ID should match the existing one but we don't check.
    let meeting_id: i64 = edit::EditMeeting::execute(edit_mutation_variables)
        .await?
        .unwrap_or(meeting_id);

    // Redirect the user back to the meeting they edited.
    return Ok(HttpResponse::Found()
        .header(LOCATION, format!("/meeting/{}", meeting_id))
        .finish());
}

/// Host selection page.
#[get("/meeting/{meeting_id}/edit/select_host")]
async fn host_selection(
    Path(meeting_id): Path<i64>,
    auth: AuthenticationCookie,
    req: HttpRequest,
) -> Result<Template, TelescopeError> {
    // Check that the user can edit this meeting.
    let viewer = auth.get_rcos_username_or_error().await?;
    if !AuthorizationFor::get(Some(viewer)).await?.can_edit_by_id(meeting_id).await? {
        return Err(TelescopeError::Forbidden);
    }

    // Get host selection.
    let data = EditHostSelection::get(meeting_id).await?;

    // Create host selection page template.
    let mut template: Template = Template::new(HOST_SELECTION_TEMPLATE);
    template.set_field("data", data);
    return template.render_into_page(&req, "Select Host").await;
}
