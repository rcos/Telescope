use crate::{
    models::confirmations::Confirmation,
    templates::{forms::registration::RegistrationPage, jumbotron, Template},
    web::RequestContext,
};
use actix_web::web::Form;
use actix_web::HttpResponse;
use lettre::EmailAddress;

/// The form used by users to sign up for an RCOS account.
#[derive(Deserialize, Debug, Clone)]
pub struct RegistrationForm {
    /// The email of the user registering for an account.
    email: EmailAddress,
}

/// Service to show the signup page. All registration requests use POST.
#[get("/register")]
pub async fn signup_page(ctx: RequestContext) -> HttpResponse {
    // if a user is logged in they cannot register for a new account.
    if ctx.logged_in().await {
        let jumbotron: Template = jumbotron::new(
            "Signed In",
            "You are already signed in. Sign out before creating a new account.",
        );

        HttpResponse::BadRequest().body(ctx.render_in_page(&jumbotron, "Registration Error").await)
    } else {
        let registration_page: Template = RegistrationPage::default().into();
        HttpResponse::Ok().body(ctx.render_in_page(&registration_page, "Sign Up").await)
    }
}

/// Service to register a new user. Respond only to POST requests.
#[post("/register")]
pub async fn registration_service(
    ctx: RequestContext,
    form: Form<RegistrationForm>,
) -> HttpResponse {
    if ctx.logged_in().await {
        let jumbotron: Template = jumbotron::new(
            "Signed In",
            "You are already signed in. Sign out before creating a new account.",
        );
        HttpResponse::BadRequest().body(ctx.render_in_page(&jumbotron, "Registration Error").await)
    } else {
        let email: String = form.email.to_string();
        let invite: Result<Confirmation, String> =
            Confirmation::invite_new(&ctx, email.clone()).await;

        if let Err(msg) = invite {
            let page: Template = RegistrationPage::error(email, msg).into();
            HttpResponse::InternalServerError().body(ctx.render_in_page(&page, "Sign Up").await)
        } else {
            let page: Template = RegistrationPage::success(email).into();
            HttpResponse::Ok().body(ctx.render_in_page(&page, "Email Sent!").await)
        }
    }
}
