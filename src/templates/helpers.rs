//! Handlebars helpers.

use handlebars::{Handlebars, Helper, Context, RenderContext, Output, HelperResult};
use crate::web::profile_for;

/// Register the custom handlebars helpers to the handlebars registry.
pub fn register_helpers(registry: &mut Handlebars) {
    registry
        .register_helper("profile_for",Box::new(profile_for_helper));
}

/// Helper function matching handlebars helper definition form to get the profile
/// url for a username.
fn profile_for_helper(
    h: &Helper<'_, '_>,
    _:&Handlebars<'_>,
    _: &Context,
    rc: &mut RenderContext<'_, '_>,
    out: &mut dyn Output
) -> HelperResult {
    // Get the first parameter
    let username: &str = h.param(0)
        // As a string
        .and_then(|v| v.value().as_str())
        // Handle errors
        .expect("profile_for helper parameter error");

    // Get the profile for this user
    let converted: String = profile_for(username);
    // Write it to the output.
    out.write(converted.as_ref())?;
    Ok(())
}
