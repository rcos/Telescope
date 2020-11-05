use crate::web::RequestContext;
use actix_web::{
    HttpResponse,
    web::Path
};
use uuid::Uuid;
use actix_web::web::Form;

/// The form sent by users to confirm an email.
#[derive(Clone, Serialize, Deserialize, Debug)]
struct ConfirmationForm {
    name: Option<String>,
    new_pass: String,
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
    Form(form): Form<ConfirmationForm>
) -> HttpResponse {
    unimplemented!()
}