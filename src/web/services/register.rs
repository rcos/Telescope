use crate::web::RequestContext;
use actix_web::HttpResponse;
use lettre::EmailAddress;
use actix_web::web::Form;

/// The form used by users to sign up for an RCOS account.
#[derive(Deserialize, Debug, Clone)]
struct RegistrationForm {
    /// The email of the user registering for an account.
    email: EmailAddress
}

/// Service to show the signup page. All registration requests use POST.
#[get("/register")]
pub async fn signup_page(ctx: RequestContext) -> HttpResponse {
    unimplemented!()
}

/// Service to register a new user. Respond only to POST requests.
#[post("/register")]
pub async fn registration_service(ctx: RequestContext, form: Form<RegistrationForm>) -> HttpResponse {
    unimplemented!()
}
