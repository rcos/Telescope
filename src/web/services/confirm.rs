use crate::{
    models::confirmations::{ConfirmNewUserError, Confirmation},
    templates::{
        forms::confirmation::{ExistingUserConf, NewUserConf},
        jumbotron::Jumbotron,
        page::Page,
    },
    web::RequestContext,
};
use actix_web::{
    http::header,
    web::{Form, Path},
    HttpResponse,
};
use uuid::Uuid;

/// The form sent to new users to confirm the account creation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewUserConfInput {
    /// The name of the user
    name: String,
    /// The password
    password: String,
    /// The confirmation of the password. This should match the password.
    confirm: String,
}

#[get("/confirm/{invite_id}")]
pub async fn confirmations_page(ctx: RequestContext, Path(invite_id): Path<Uuid>) -> HttpResponse {
    let confirmation = Confirmation::get_by_id(ctx.get_db_conn().await, invite_id)
        .await
        .expect("Error getting confirmation.");

    // handle missing or expired confirmation.
    if let Some(confirmation) = confirmation {
        if confirmation.creates_user() {
            let form = NewUserConf::new(confirmation);
            let page = Page::of("Create account", &form, &ctx).await;
            HttpResponse::Ok().body(ctx.render(&page))
        } else {
            let error_message = confirmation.clone().confirm_existing(&ctx).await.err();

            // make page title
            let errored = error_message.is_some();
            let page_title = if errored { "Error" } else { "RCOS" };

            // make confirmation page
            let conf = ExistingUserConf::new(confirmation, error_message);
            let rendered = ctx.render_in_page(&conf, page_title).await;

            return if errored {
                HttpResponse::InternalServerError().body(rendered)
            } else {
                HttpResponse::Ok().body(rendered)
            };
        }
    } else {
        let page = Jumbotron::jumbotron_page(
            &ctx,
            "Not Found",
            "Invite Not Found",
            format!(
                "Could not find confirmation {}. It may have expired.",
                invite_id
            ),
        )
        .await;
        HttpResponse::NotFound().body(page)
    }
}

#[post("/confirm/{invite_id}")]
pub async fn confirm(
    ctx: RequestContext,
    Path(invite_id): Path<Uuid>,
    Form(form): Form<NewUserConfInput>,
) -> HttpResponse {
    let confirmation = Confirmation::get_by_id(ctx.get_db_conn().await, invite_id)
        .await
        .expect("Error getting confirmation.");

    // handle missing or expired confirmation.
    if let Some(confirmation) = confirmation {
        if confirmation.creates_user() {
            let NewUserConfInput {
                name,
                password,
                confirm,
            } = form;

            // check that the pasword isn't way too long. This is unlikely to happen but
            // serves to prevent bad actors from locking up the server by submitting super long
            // passwords.

            const MAX_PASS_LEN: usize = 10_000;

            if password.len() > MAX_PASS_LEN {
                let mut form = NewUserConf::new(confirmation).name(&name);
                form.password = form
                    .password
                    .error("Password too long. Please stay under 10,000 bytes.");
                return HttpResponse::BadRequest().body(ctx.render_in_page(&form, "Error").await);
            }

            if confirm.len() > MAX_PASS_LEN {
                let mut form = NewUserConf::new(confirmation).name(&name);
                form.confirm_password = form
                    .confirm_password
                    .error("Password too long. Please stay under 10,000 bytes.");
                return HttpResponse::BadRequest().body(ctx.render_in_page(&form, "Error").await);
            }

            // check that the password and the confirm password are the same.
            if password != confirm {
                let mut form = NewUserConf::new(confirmation).name(&name);
                form.password = form
                    .password
                    .error("Password does not match confirm password.");
                return HttpResponse::BadRequest().body(ctx.render_in_page(&form, "Error").await);
            }

            let res = confirmation
                .clone()
                .confirm_new(&ctx, name.clone(), password)
                .await;

            match res {
                Ok(new_user) => {
                    // log the user in.
                    // in the future we should probably have a better form
                    // of user identity than just the uuid.
                    ctx.identity().remember(new_user.id_str());

                    let profile_url = format!("/profile/{}", new_user.id_str());

                    return HttpResponse::Found()
                        .header(header::LOCATION, profile_url)
                        .finish();
                }
                Err(ConfirmNewUserError::BadPassword(reqs)) => {
                    let mut form = NewUserConf::new(confirmation).name(name);
                    form.password = form.password.error(
                        reqs.get_error_string()
                            .expect("Could not get error string for password requirements"),
                    );
                    HttpResponse::BadRequest().body(ctx.render_in_page(&form, "Error").await)
                }
                Err(ConfirmNewUserError::Other(msg)) => {
                    error!("Could not confirm new user: {}", msg);
                    let jumbotron = Jumbotron::jumbotron_page(
                        &ctx,
                        "Error",
                        "500 - Internal Server Error",
                        "We encountered an error while processing your request. Please try again \
                        If you continue to have issues, please make a github issue and/or contact a server \
                        administrator."
                    ).await;
                    HttpResponse::InternalServerError().body(jumbotron)
                }
            }
        } else {
            let error_message = format!(
                "Confirmation {} is already linked to an existing user",
                invite_id
            );

            let page =
                Jumbotron::jumbotron_page(&ctx, "Cannot create user", "Bad request", error_message)
                    .await;

            return HttpResponse::BadRequest().body(page);
        }
    } else {
        HttpResponse::NotFound().body(
            Jumbotron::jumbotron_page(
                &ctx,
                "Not Found",
                "Invite Not Found",
                format!(
                    "Could not find confirmation {}. It may have expired.",
                    invite_id
                ),
            )
            .await,
        )
    }
}
