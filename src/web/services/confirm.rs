use crate::web::RequestContext;
use actix_web::web::Form;
use actix_web::{web::Path, HttpResponse};
use uuid::Uuid;
use crate::models::Confirmation;

/// The form sent to new users to confirm the account creation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfirmationForm {
    name: String,
    pass: String,
    confirm_pass: String,
}

#[get("/confirm/{invite_id}")]
pub async fn confirmations_page(ctx: RequestContext, Path(invite_id): Path<Uuid>) -> HttpResponse {
    let confirmation = Confirmation::get_by_id(
        ctx.get_db_conn().await,
        invite_id
    ).await.expect("Error getting confirmation.");

}

#[post("/confirm/{invite_id}")]
pub async fn confirm(
    ctx: RequestContext,
    Path(invite_id): Path<Uuid>,
    Form(form): Form<ConfirmationForm>,
) -> HttpResponse {
    unimplemented!()
}
