use crate::web::RequestContext;
use actix_web::HttpResponse;
use actix_web::web::Form;
use crate::templates::recovery::PasswordRecoveryPage;
use crate::templates::page::Page;

/// Form submitted by users to recovery service to set a new password.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordRecoveryForm {
    email: String,
}

/// The password recovery page.
#[get("/forgot")]
pub async fn forgot_page(ctx: RequestContext) -> HttpResponse {
    let form = PasswordRecoveryPage::default();
    let page = Page::of("Forgot Password", &form, &ctx).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(ctx.render(&page))
}

#[post("/forgot")]
pub async fn recovery_service(ctx: RequestContext, form: Form<PasswordRecoveryForm>) -> HttpResponse {
    unimplemented!()
}
