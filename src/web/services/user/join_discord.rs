//! Page and service to let users into RCOS Discord and give them the verified role.

use crate::api::discord::{global_discord_client, rcos_discord_verified_role_id};
use crate::api::rcos::users::discord_whois::DiscordWhoIs;
use crate::error::TelescopeError;

use crate::env::global_config;
use crate::web::services::auth::identity::AuthenticationCookie;
use actix_web::HttpResponse;
use reqwest::header::LOCATION;
use serenity::model::prelude::RoleId;

/// Let users into the RCOS discord.
#[get("/join_discord")]
pub async fn handle(auth: AuthenticationCookie) -> Result<HttpResponse, TelescopeError> {
    // Get the authenticated user id.
    let user_id = auth.get_user_id_or_error().await?;

    // Get Discord access token.
    let discord = auth.get_discord();

    // Ensure that Discord access is available.
    if discord.is_none() {
        return Err(TelescopeError::BadRequest {
            header: "Could not join RCOS Discord".to_string(),
            message: "Please log out and then login with Discord to continue".to_string(),
            show_status_code: false,
        });
    }

    // Unwrap Discord token.
    let discord = discord.unwrap();
    // Get Discord user ID.
    let discord_user_id: u64 = discord
        .get_discord_id()
        .await?
        .as_str()
        .parse::<u64>()
        .expect("Malformed Discord user ID.");

    // Ensure that the authenticated user has RPI CAS linked and get the RCS ID.
    let rcs_id: Option<String> = auth.get_rcs_id().await?;
    if rcs_id.is_none() {
        return Err(TelescopeError::BadRequest {
            header: "Could not join RCOS Discord".to_string(),
            message: "RPI CAS must be linked to join RCOS Discord.".to_string(),
            show_status_code: false,
        });
    }

    // Unwrap the RCS ID.
    let rcs_id: String = rcs_id.unwrap();
    // Add the user to the server.

    // Get user info to make Discord nickname.
    let user_info = DiscordWhoIs::send(discord_user_id)
        .await?
        .get_user()
        .expect("User must exist");

    // Extract some fields.
    let fname: &str = user_info.first_name.as_str();
    let lname: &str = user_info.last_name.as_str();
    let cohort: Option<i64> = user_info.cohort.clone();
    // Use only the last two digits of the graduation year.
    let cohort_format = cohort
        .map(|cohort| format!("'{} ", (cohort + 4) % 100))
        .unwrap_or("".to_string());

    // Format nickname
    let nickname: String = format!(
        "{} {} {}({})",
        // Limit the first name to 20 character to avoid passing Discord nickname limits.
        fname
            // Character iterator.
            .chars()
            // Add indices.
            .zip(0..)
            // Filter to the first 20 chars.
            .filter(|(_, ind)| *ind < 20)
            // Strip away the index.
            .map(|(c, _)| c)
            // Collect to string.
            .collect::<String>(),
        lname.chars().next().unwrap(),
        cohort_format,
        rcs_id
    );

    // Get RCOS Discord Verified role ID if possible. If not, user empty role list.
    let verified_role: RoleId = rcos_discord_verified_role_id()
        .await?
        .ok_or(TelescopeError::ise("Could not get Verified role ID."))?;

    // Add user to Discord with verified role and nickname.
    discord
        .add_to_rcos_guild(Some(nickname), vec![verified_role])
        .await?;

    // If user was already in the discord, they may not have the verified role, and the
    // previous call will do nothing. Make an additional call here to add the verified role
    // to the user if they don't already have it.
    // See https://github.com/rcos/Telescope/issues/236.

    // Get the rcos discord guild ID.
    let rcos_discord_guild = global_config().discord_config.rcos_guild_id();

    // Make the call to add the verified role
    global_discord_client()
        .add_member_role(rcos_discord_guild, discord_user_id, verified_role.0)
        .await
        .map_err(TelescopeError::serenity_error)?;

    // On success, redirect user back to their profile.
    Ok(HttpResponse::Found()
        .header(LOCATION, format!("/user/{}", user_id))
        .finish())
}
