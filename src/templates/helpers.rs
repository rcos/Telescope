//! Handlebars helpers.

use crate::api::rcos::meetings::MeetingType;
use crate::api::rcos::users::UserRole;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use pulldown_cmark::{Options as MarkdownOptions, Parser as MarkdownParser};
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

/// Register the custom handlebars helpers to the handlebars registry.
pub fn register_helpers(registry: &mut Handlebars) {
    registry.register_helper("format_date", wrap_helper(format_date_helper));
    registry.register_helper("format_time", wrap_helper(format_time_helper));
    registry.register_helper(
        "format_meeting_type",
        wrap_helper(format_meeting_type_helper),
    );
    registry.register_helper("format_user_role", wrap_helper(format_user_role));
    registry.register_helper("domain_of", wrap_helper(domain_of_helper));
    registry.register_helper("url_encode", wrap_helper(url_encode_helper));
    registry.register_helper("render_markdown", wrap_helper(markdown_renderer_helper));
}

/// Wrap a two-argument helper function into a helper object to add to the
/// handlebars registry.
fn wrap_helper<F>(helper_fn: F) -> Box<dyn HelperDef + Send + Sync>
where
    F: for<'reg, 'rc> Fn(&Helper<'reg, 'rc>, &mut dyn Output) -> HelperResult
        + Send
        + Sync
        + 'static,
{
    // Create the closure that implements the HelperDef trait
    let closure = move |h: &Helper,
                        _: &Handlebars,
                        _: &Context,
                        _: &mut RenderContext,
                        out: &mut dyn Output| { helper_fn(h, out) };

    // Box and return
    return Box::new(closure);
}

/// Handlebars helper to properly format a meeting type.
fn format_meeting_type_helper(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Extract the parameter
    let input: MeetingType = h
        .param(0)
        .and_then(|param| {
            // Deserialize to meeting type
            serde_json::from_value::<MeetingType>(param.value().clone()).ok()
        })
        // Convert and handle error
        .ok_or(RenderError::new(
            "format_meeting_type expects one meeting_type parameter",
        ))?;

    // Write the display formatting to the output.
    out.write(input.to_string().as_str())?;
    Ok(())
}

/// Handlebars helper to format dates. This should be able to accept a value of
/// either a date or a timestamp or a timestamp with timezone, and format the date
/// as "Month Day, Year".
fn format_date_helper(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Get the first parameter.
    let input: &str = h
        .param(0)
        // Get the value of the parameter
        .and_then(|param| param.value().as_str())
        // Handle missing parameter.
        .ok_or(RenderError::new(
            "format_date helper requires one string parameter",
        ))?;

    // If the input is a timestamp with timezone
    if let Ok(timestamp) = input.parse::<DateTime<Local>>() {
        // Format the date properly.
        let formatted: String = timestamp
            // Format
            .format("%B %_d, %Y")
            .to_string();
        // Write to output and return.
        out.write(formatted.as_str())?;
        return Ok(());
    }

    // If the input is a naive timestamp
    if let Ok(timestamp) = input.parse::<NaiveDateTime>() {
        let formatted: String = timestamp.format("%B %_d, %Y").to_string();
        out.write(formatted.as_str())?;
        return Ok(());
    }

    // If the input is just a date
    let formatted = input
        .parse::<NaiveDate>()
        // If it fails to parse, the parameter is malformed.
        .map_err(|_| RenderError::new("format_date helper expects date or timestamp"))?
        // Format
        .format("%B %_d, %Y")
        // Convert to string.
        .to_string();
    // Write to output
    out.write(formatted.as_str())?;
    // Return ok
    Ok(())
}

/// Handlebars helper to format time information.
fn format_time_helper(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Extract the input parameter
    let input: &str = h
        .param(0)
        // Convert to string
        .and_then(|p| p.value().as_str())
        // Error on no input
        .ok_or(RenderError::new(
            "format_time helper expects one string parameter.",
        ))?;

    // Try to parse a timestamp
    if let Ok(timestamp) = input.parse::<DateTime<Local>>() {
        let formatted: String = timestamp
            // Format date.
            .format("%_I:%M %P")
            .to_string();
        out.write(formatted.as_str())?;
        return Ok(());
    }

    // Next try a naive timestamp
    if let Ok(timestamp) = input.parse::<NaiveDateTime>() {
        let formatted: String = timestamp.format("%_I:%M %P").to_string();
        out.write(formatted.as_str())?;
        return Ok(());
    }

    // Lastly try just a time.
    let formatted: String = input
        .parse::<NaiveTime>()
        // Convert and propagate error if necessary
        .map_err(|_| RenderError::new("format_time helper expects a date or timestamp"))?
        // Format the time.
        .format("%_I:%M %P")
        .to_string();
    out.write(formatted.as_str())?;
    Ok(())
}

/// Handlebars helper to extract the domain and subdomain of a URL.
fn domain_of_helper(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Extract the parameter.
    let host: String = h
        .param(0)
        // The parameter should be a string.
        .and_then(|param| param.value().as_str())
        // Parse parameter into URL
        .and_then(|s: &str| Url::parse(s).ok())
        // Extract the host from the URL
        .and_then(|url: Url| url.host_str().map(|s| s.to_string()))
        // If there are any issues propagate an error.
        .ok_or(RenderError::new(
            "domain_of helper expects one URL argument",
        ))?;

    // Write to output
    out.write(host.as_str())?;
    Ok(())
}

/// Helper to render a user role variant via Display trait.
fn format_user_role(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Extract the parameter
    let user_role = h
        // One parameter required.
        .param(0)
        // Parse to user role
        .and_then(|param| serde_json::from_value::<UserRole>(param.value().clone()).ok())
        .ok_or(RenderError::new("format_user_role expects a user's role"))?;

    out.write(user_role.to_string().as_str())?;
    Ok(())
}

/// Helper to urlencode a query string.
/// This accepts a series of hash arguments and encodes all of them.
fn url_encode_helper(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Map the Helper's hash params into just JSON values (we do not use the paths).
    let map: HashMap<&str, &Value> = h
        .hash()
        .iter()
        // Use just the value.
        .map(|(key, val)| (*key, val.value()))
        // Collect into hashmap.
        .collect();

    // Url-encode the map.
    let encoded: String = serde_urlencoded::to_string(map)
        // Report any errors.
        .map_err(|e| RenderError::new(format!("Could not url-encode object: {}", e)))?;

    // Write the url-encoded string.
    out.write(encoded.as_str())?;
    Ok(())
}

/// Helper to parse and render a markdown string.
fn markdown_renderer_helper(h: &Helper<'_, '_>, out: &mut dyn Output) -> HelperResult {
    // Expect one parameter with the markdown payload.
    let markdown_source: &str =
        h.param(0)
            .and_then(|param| param.value().as_str())
            .ok_or(RenderError::new(
                "render_markdown expects a markdown string parameter.",
            ))?;
    // Make a new parser with all options enabled.
    let parser = MarkdownParser::new_ext(markdown_source, MarkdownOptions::all());
    // Make and write an html buffer with the rendered markdown.
    // Set the initial capacity at at least the length of the source markdown.
    let mut buffer = String::with_capacity(markdown_source.len());
    pulldown_cmark::html::push_html(&mut buffer, parser);
    // Write the rendered HTML to the handlebars output.
    out.write(buffer.as_str())?;
    return Ok(());
}
