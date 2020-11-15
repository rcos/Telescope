use actix_web::{web::Form, HttpResponse};

use crate::{
    models::emails::Email,
    templates::{forms::recovery::PasswordRecoveryPage, page::Page},
    web::RequestContext,
};

/// Form submitted by users to recovery service to set a new password.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordRecoveryForm {
    email: String,
}

/// The password recovery page.
#[get("/forgot")]
pub async fn forgot_page(ctx: RequestContext) -> HttpResponse {
    let form = PasswordRecoveryPage::new();
    let rendered = ctx.render_in_page(&form, "Forgot Password").await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[post("/forgot")]
pub async fn recovery_service(
    ctx: RequestContext,
    form: Form<PasswordRecoveryForm>,
) -> HttpResponse {
    let email: &str = &form.email;
    let mut form_page = PasswordRecoveryPage::new().email(email);
    let database_result = Email::get_user_from_db_by_email(
        ctx.get_db_conn().await,
        email.to_string()
    ).await;
    if let Some(target_user) = database_result {
        // get the user's emails.
        let emails = target_user.get_emails_from_db(ctx.get_db_conn().await).await;

    } else {
        form_page = form_page.error("Email Not Found");
        let page = ctx.render_in_page(&form_page, "Forgot Password").await;
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(page)
    }
}
