//! Handlebars helpers.

use handlebars::{Handlebars, Helper, Context, RenderContext, Output, HelperResult, HelperDef};
use crate::web::profile_for;
use serde_json::Value;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};

/// Register the custom handlebars helpers to the handlebars registry.
pub fn register_helpers(registry: &mut Handlebars) {
    registry.register_helper("profile_for",wrap_helper(profile_for_helper));
    registry.register_helper("format_date", wrap_helper(format_date_helper));
}

/// Wrap a two-argument helper function into a helper object to add to the
/// handlebars registry.
fn wrap_helper<F>(helper_fn: F) -> Box<dyn HelperDef + Send + Sync>
    where F: for<'reg, 'rc> Fn(&Helper<'reg, 'rc>, &mut dyn Output) -> HelperResult + Send + Sync + 'static
{
    // Create the closure that implements the HelperDef trait
    let closure = move |h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output| {
        helper_fn(h, out)
    };

    // Box and return
    return Box::new(closure);
}

/// Helper function matching handlebars helper definition form to get the profile
/// url for a username.
fn profile_for_helper(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Get the first parameter
    let username: &str = h.param(0)
        // As a string
        .and_then(|v| v.value().as_str())
        // Handle errors
        .expect("profile_for helper parameter should be string");

    // Get the profile for this user
    let converted: String = profile_for(username);
    // Write it to the output.
    out.write(converted.as_str())?;
    Ok(())
}

/// Handlebars helper to format dates. This should be able to accept a value of
/// either a date or a timestamp or a timestamp with timezone, and format the date
/// as "Month Day, Year".
fn format_date_helper(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Get the first parameter.
    let input: &str = h.param(0)
        // Get the value of the parameter
        .and_then(|param| param.value().as_str())
        // Handle missing parameter.
        .expect("format_date helper requires string parameter");

    // If the input is a timestamp with timezone
    if let Ok(timestamp) = serde_json::from_str::<DateTime<Local>>(input) {
        // Format the date properly.
        let formatted: String = timestamp.date().format("%B %_d, %Y").to_string();
        // Write to output and return.
        out.write(formatted.as_str())?;
        return Ok(());
    }

    // If the input is a naive timestamp
    if let Ok(timestamp) = serde_json::from_str::<NaiveDateTime>(input) {
        let formatted: String = timestamp.date().format("%B %_d, %Y").to_string();
        out.write(formatted.as_str())?;
        return Ok(());
    }

    // If the input is just a date
    let formatted = serde_json::from_str::<NaiveDate>(input)
        // If it fails to parse, the parameter is malformed.
        .expect("format_date parameter")
        // Format
        .format("%B %_d, %Y")
        // Convert to string.
        .to_string();
    // Write to output
    out.write(formatted.as_str())?;
    // Return ok
    Ok(())
}
