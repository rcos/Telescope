use actix_web::{web::Path, HttpResponse};

use uuid::Uuid;

use crate::web::RequestContext;

/// The service to display a user profile. The user is specified by the id in the
/// request path.
#[get("/profile/{uid}")]
pub async fn profile(ctx: RequestContext, Path(t_uid): Path<Uuid>) -> HttpResponse {
    unimplemented!()
}

#[get("/profile/{uid}/settings")]
pub async fn settings_page(ctx: RequestContext, Path(t_uid): Path<Uuid>) -> HttpResponse {
    unimplemented!()
}

#[post("/profile/{uid}/settings")]
pub async fn settings_update(ctx: RequestContext, Path(t_uid): Path<Uuid>) -> HttpResponse {
    unimplemented!()
}

#[get("/profile/{uid}/add_email")]
pub async fn add_email_page(ctx: RequestContext, Path(t_uid): Path<Uuid>) -> HttpResponse {
    unimplemented!()
}

#[post("/profile/{uid}/add_email")]
pub async fn add_email(ctx: RequestContext, Path(t_uid): Path<Uuid>) -> HttpResponse {
    unimplemented!()
}