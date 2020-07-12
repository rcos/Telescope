use crate::web::PageContext;
use actix_web::web::Form;
use actix_web::HttpResponse;
use std::collections::HashMap;

/// Guarded to only post requests.
pub fn auth_service(pc: PageContext, login: Form<HashMap<String, String>>) -> HttpResponse {
    let session = pc.session();
    dbg!(login);
    unimplemented!()
}