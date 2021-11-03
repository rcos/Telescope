use crate::api::rcos::users::{delete::DeleteUser, profile::Profile, UserAccountType};
use crate::error::TelescopeError;
use crate::templates::{forms::FormTemplate, jumbotron, Template};
use crate::web::services::auth::identity::{AuthenticationCookie, Identity};
use actix_web::{http::header::LOCATION, web::Form, HttpRequest, HttpResponse};
use crate::api::discord::global_discord_client;
use crate::api::rcos::users::accounts::lookup::AccountLookup;
use crate::env::global_config;

/// Confirmation form to delete the profile
#[get("/profile_delete")]
pub async fn confirm_delete(auth: AuthenticationCookie) -> Result<FormTemplate, TelescopeError> {
    let username = auth.get_rcos_username_or_error().await?;
    let profiledata = Profile::for_user(username.clone(), Some(username)).await?;

    let mut form = FormTemplate::new("user/delete", "Delete confirmation");
    form.template = json!(profiledata);

    Ok(form)
}

#[post("/profile_delete")]
pub async fn profile_delete(req: HttpRequest, identity: Identity) -> Result<Template, TelescopeError> {
    // Get the viewer's RCOS username.
    let rcos_username: String = identity
        .get_rcos_username()
        .await?
        .ok_or(TelescopeError::NotAuthenticated)?;

    // Check if the viewer has a discord account linked.
    let discord_id: Option<u64> = AccountLookup::send(rcos_username.clone(), UserAccountType::Discord)
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
    DeleteUser::execute(rcos_username).await?;

    // Clear the user's cookies.
    identity.forget();

    // Show the user a jumbotron indicating account deletion.
    jumbotron::new("Account deletion", "Your account was deleted successfully.")
            .render_into_page(&req, "Account deletion")
            .await
}
