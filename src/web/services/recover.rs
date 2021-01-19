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

#[get("/recover/{rid}")]
pub async fn recover_form(ctx: RequestContext, Path(r_id): Path<Uuid>) -> HttpResponse {
    unimplemented!()
}


#[post("/recovery/{rid}")]
pub async fn recovery(ctx: RequestContext, Path(r_id): Path<Uuid>, Form(f): Form<RecoveryForm>) -> HttpResponse {
    unimplemented!()
}
