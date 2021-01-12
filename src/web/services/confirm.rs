use crate::{
    models::{
        confirmations::{ConfirmNewUserError, Confirmation},
        password_requirements::PasswordRequirements,
        users::User,
    },
    templates::{
        forms::{common::text_field, confirmation},
        jumbotron, page, Template,
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
            let form: Template = confirmation::for_conf(&confirmation);
            let page: Template = page::of(&ctx, "Create account", &form).await;
            HttpResponse::Ok().body(ctx.render(&page))
        } else {
            let error_message = confirmation.confirm_existing(&ctx).await.err();

            // make page title
            let errored = error_message.is_some();
            let page_title = if errored { "Error" } else { "RCOS" };

            // make confirmation page
            let conf: Template =
                confirmation::for_conf(&confirmation).field(confirmation::ERROR, error_message);
            let rendered: String = ctx.render_in_page(&conf, page_title).await;

            return if errored {
                HttpResponse::InternalServerError().body(rendered)
            } else {
                HttpResponse::Ok().body(rendered)
            };
        }
    } else {
        let err_msg: String = format!(
            "Could not find confirmation {}. It may have expired.",
            invite_id
        );
        let jumbo: Template = jumbotron::new("Invite Not Found", err_msg);
        HttpResponse::NotFound().body(ctx.render_in_page(&jumbo, "Not Found").await)
    }
}

#[post("/confirm/{invite_id}")]
pub async fn confirm(
    ctx: RequestContext,
    Path(invite_id): Path<Uuid>,
    Form(form): Form<NewUserConfInput>,
) -> HttpResponse {
    // Get confirmation record from database.
    let confirmation: Option<Confirmation> =
        Confirmation::get_by_id(ctx.get_db_conn().await, invite_id)
            .await
            .expect("Error getting confirmation.");

    // Handle missing confirmation.
    if confirmation.is_none() {
        let err_msg: String = format!(
            "Could not find confirmation {}. It may have expired.",
            invite_id
        );
        let jumbo: Template = jumbotron::new("Invite Not Found", err_msg);
        return HttpResponse::NotFound().body(ctx.render_in_page(&jumbo, "Not Found").await);
    }

    let confirmation: Confirmation = confirmation.unwrap();

    // Make sure that the confirmation creates a user. We do not accept post requests for existing
    // users.
    if !confirmation.creates_user() {
        let error_message: String = format!(
            "Confirmation {} is already linked to an existing user",
            invite_id
        );

        let page: String =
            jumbotron::rendered_page(&ctx, "Cannot create user", "Bad request", error_message)
                .await;

        return HttpResponse::BadRequest().body(page);
    }

    // Destructure form.
    let NewUserConfInput {
        name,
        password,
        confirm,
    } = form;

    // Form to return if errors occur.
    let mut form_err: Template = confirmation::for_conf(&confirmation);
    form_err[confirmation::NAME][text_field::PREFILL_FIELD] = name.as_str().into();

    // Check that the password and the confirm password are the same.
    if password != confirm {
        form_err[confirmation::PASSWORD][text_field::ERROR_FIELD] =
            "Password does not match confirm password.".into();
        return HttpResponse::BadRequest().body(ctx.render_in_page(&form_err, "Error").await);
    }

    // Try to confirm the new user.
    let res: Result<User, ConfirmNewUserError> =
        confirmation.confirm_new(&ctx, name.clone(), password).await;

    match res {
        // Success
        Ok(new_user) => {
            // log the user in.
            // in the future we should probably have a better form
            // of user identity than just the uuid.
            ctx.identity().remember(new_user.id_str());

            let profile_url: String = format!("/profile/{}", new_user.id_str());

            return HttpResponse::Found()
                .header(header::LOCATION, profile_url)
                .finish();
        }

        // Handle bad password.
        Err(ConfirmNewUserError::BadPassword(reqs)) => {
            let err_str: String = reqs
                .get_error_string()
                .expect("Could not get error string for password requirements");

            form_err[confirmation::PASSWORD][text_field::ERROR_FIELD] = err_str.into();

            HttpResponse::BadRequest().body(ctx.render_in_page(&form_err, "Error").await)
        }

        // Handle other confirmation error.
        Err(ConfirmNewUserError::Other(msg)) => {
            error!("Could not confirm new user: {}", msg);

            let jumbotron: String = jumbotron::rendered_page(
                &ctx,
                "Error",
                "500 - Internal Server Error",
                "We encountered an error while processing your request. Please try again \
                If you continue to have issues, please make a github issue and/or contact a server \
                administrator.",
            )
            .await;

            HttpResponse::InternalServerError().body(jumbotron)
        }
    }
}
