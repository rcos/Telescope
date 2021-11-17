use crate::api::discord::global_discord_client;
use crate::api::rcos::users::accounts::lookup::AccountLookup;
use crate::api::rcos::users::{delete::DeleteUser, profile::Profile, UserAccountType};
use crate::env::global_config;
use crate::error::TelescopeError;
use crate::templates::{forms::FormTemplate, jumbotron, Template};
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};
use actix_web::HttpRequest;

/// Confirmation form to delete the profile
#[get("/profile_delete")]
pub async fn confirm_delete(auth: AuthenticationCookie) -> Result<FormTemplate, TelescopeError> {
    let user_id = auth.get_user_id_or_error().await?;
    // The viewer and target are both the same user ID.
    let profile_data = Profile::for_user(user_id, Some(user_id)).await?;

    let mut form = FormTemplate::new("user/delete", "Delete confirmation");
    form.template = json!(profile_data);

    Ok(form)
}

#[post("/profile_delete")]
pub async fn profile_delete(
    req: HttpRequest,
    identity: Identity,
) -> Result<Template, TelescopeError> {
    // Get the viewer's RCOS user ID.
    let user_id = identity
        .get_user_id()
        .await?
        .ok_or(TelescopeError::NotAuthenticated)?;

    // Check if the viewer has a discord account linked.
    let discord_id: Option<u64> = AccountLookup::send(user_id, UserAccountType::Discord)
        .await?
        .and_then(|string| string.as_str().parse::<u64>().ok());

    // If there is one, kick it from the RCOS Discord.
    if let Some(discord_id) = discord_id {
        // Get the RCOS Discord Guild ID.
        let rcos_guild = global_config().discord_config.rcos_guild_id();

        // Kick the user from the RCOS guild.
        global_discord_client()
            .kick_member(rcos_guild, discord_id)
            .await
            .map_err(TelescopeError::serenity_error)?;
    }

    // Execute the user deletion.
    DeleteUser::execute(user_id).await?;

    // Clear the user's cookies.
    identity.forget();

    // Show the user a jumbotron indicating account deletion.
    jumbotron::new("Account deletion", "Your account was deleted successfully.")
        .render_into_page(&req, "Account deletion")
        .await
}
