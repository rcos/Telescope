use crate::web::RequestContext;
use actix_web::web::Form;
use actix_web::{web::Path, HttpResponse};
use uuid::Uuid;

/// The form sent to new users to confirm the account creation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfirmationForm {
    name: String,
    pass: String,
    confirm_pass: String,
}

#[get("/confirm/{invite_id}")]
pub async fn confirmations_page(ctx: RequestContext, Path(invite_id): Path<Uuid>) -> HttpResponse {
    unimplemented!()
}

#[post("/confirm/{invite_id}")]
pub async fn confirm(
    ctx: RequestContext,
    Path(invite_id): Path<Uuid>,
    Form(form): Form<ConfirmationForm>,
) -> HttpResponse {
    unimplemented!()
}
