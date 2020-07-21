use crate::web::RequestContext;
use actix_web::web::Form;
use actix_web::HttpResponse;
use std::collections::HashMap;

/// Guarded to only post requests.
pub async fn login_service(req_ctx: RequestContext, login: Form<HashMap<String, String>>,) -> HttpResponse {
    let session = req_ctx.session();

    dbg!(login);
    unimplemented!()
}
