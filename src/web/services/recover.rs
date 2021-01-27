use crate::web::RequestContext;
use actix_web::web::{Path, Form};
use uuid::Uuid;
use actix_web::HttpResponse;

/// The data submitted when recovering an account. Both strings should be identical.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryForm {
    new_pass: String,
    confirm: String,
}

/// The page to display the form to set a new password.
#[get("/recover/{rid}")]
pub async fn recover_form(ctx: RequestContext, Path(r_id): Path<Uuid>) -> HttpResponse {
    unimplemented!()
}

/// The page to receive requests to set a new password and either change the password or
/// or respond with the partial form and feedback.
#[post("/recovery/{rid}")]
pub async fn recovery(ctx: RequestContext, Path(r_id): Path<Uuid>, Form(f): Form<RecoveryForm>) -> HttpResponse {
    unimplemented!()
}
