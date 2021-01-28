use crate::web::RequestContext;
use actix_web::web::{Path, Form};
use uuid::Uuid;
use actix_web::HttpResponse;

/// The data submitted when recovering an account. Both strings should be identical.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryForm {
    new_pass: String,
    confirm_pass: String,
}

/// The page to display the form to set a new password.
/// The id in the path should be an id in the lost_passwords table.
#[get("/recover/{rid}")]
pub async fn recover_form(ctx: RequestContext, Path(r_id): Path<Uuid>) -> HttpResponse {
    // Fist get the recovery record from the lost passwords table (make sure it exists)
    // Double check that the recovery record hasn't expired.
    // Then make the empty form and show it to the user.
    unimplemented!()
}

/// The page to receive requests to set a new password and either change the password or
/// or respond with the partial form and feedback.
#[post("/recovery/{rid}")]
pub async fn recovery(ctx: RequestContext, Path(r_id): Path<Uuid>, Form(f): Form<RecoveryForm>) -> HttpResponse {
    unimplemented!()
}
