//! Form to create a meeting.

use crate::templates::forms::common::text_field::TextField;
use crate::templates::forms::FormTemplate;
use chrono::{NaiveDate, NaiveTime};
use std::fmt::Display;

/// The path to the handlebars template to create a meeting.
const TEMPLATE_PATH: &'static str = "forms/meeting/create";

/// The title of the page to create a meeting.
const PAGE_TITLE: &'static str = "Create Meeting";

/// The form field for the meetings title.
pub const TITLE: &'static str = "title";

/// The form field for the meetings start date.
pub const START_DATE: &'static str = "start_date";

/// The form field for the meetings end date.
pub const END_DATE: &'static str = "end_date";

/// The form field for the meetings start time.
pub const START_TIME: &'static str = "start_time";

/// The form field for the meetings end time.
pub const END_TIME: &'static str = "end_time";

/// Create the title field.
///
/// The title has no limits on its input currently.
fn title_field() -> TextField {
    TextField::new(TITLE, |input: Option<&String>| {
        // Create an empty title field.
        let mut result: TextField = title_field();
        // If there is input
        if let Some(title) = input {
            // Trim whitespace
            let trimmed: &str = title.trim();
            // If it's not empty, put it in the text field.
            if !trimmed.is_empty() {
                result.value = Some(trimmed.to_string());
            }
        }

        return result;
    })
}

/// Create a date/time field with a given name.
fn chrono_field<T: Display + 'static, E: 'static>(
    name: &'static str,
    parse: fn(&str) -> Result<T, E>,
) -> TextField {
    // Convert the name.
    TextField::new(name.to_string(), move |input: Option<&String>| {
        // Create a result version of this field to modify.
        let mut result: TextField = chrono_field::<T, E>(name, parse);

        // This field is required, so return an error on no input.
        if let Some(value) = input.map(|input| input.trim()) {
            // There is a value, lets try to parse it.
            match parse(value) {
                Ok(parsed) => {
                    result.value = Some(parsed.to_string());
                    result.is_valid = Some(true);
                    result.success = Some("Looks good!".to_string());
                }
                Err(_) => {
                    result.value = Some(value.to_string());
                    result.is_valid = Some(false);
                    result.error = Some("Malformed input".into());
                }
            }
        } else {
            // No input so error,
            result.is_valid = Some(false);
            result.error = Some("This field is required".into());
        }
        return result;
    })
}

/// Create a date field with a given name.
fn date_field(name: &'static str) -> TextField {
    chrono_field::<NaiveDate, _>(name, |input: &str| input.parse::<NaiveDate>())
}

/// Create a time field with a given name.
fn time_field(name: &'static str) -> TextField {
    chrono_field::<NaiveTime, _>(name, |input: &str| {
        NaiveTime::parse_from_str(input, "%H:%M")
    })
}

/// Create a meeting creation form object.
pub fn make() -> FormTemplate {
    // Create the form
    let mut f = FormTemplate::new(TEMPLATE_PATH, PAGE_TITLE);
    // Add the title field
    f.add_text_field(title_field());
    // Add the date and time fields
    f.add_text_field(date_field(START_DATE));
    f.add_text_field(time_field(START_TIME));
    f.add_text_field(date_field(END_DATE));
    f.add_text_field(time_field(END_TIME));
    // Return the form.
    return f;
}
