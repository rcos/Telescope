use actix_web::{web::Form, HttpResponse};

use crate::{
    models::emails::Email,
    templates::{forms::recovery::PasswordRecoveryPage, page::Page},
    web::RequestContext,
};
use crate::models::recoveries::Recovery;
use crate::templates::emails::recovery_email::PasswordRecoveryEmail;

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
        let emails: Vec<String> = target_user
            .get_emails_from_db(ctx.get_db_conn().await)
            .map(|e: Email| e.email)
            .await;

        // make a recovery record
        let recovery = Recovery::for_user(&target_user);

        // make recovery link
        let link = ctx
            .request()
            .uri()
            .authority()
            .map(|a| format!("https://{}", a.as_str()))
            // since lettre doesn't currently store the messages in a human readable
            // format in stub or file transport, we log the generated address here.
            .map(|url| {
                trace!("Generated recovery URL: {}", url);
                url
            })
            .expect("Could not make recovery URL.");

        // make the recovery email
        let recovery_email =
            PasswordRecoveryEmail::new(recovery.clone(), link);

        let email = lettre_email::Email::builder()
            .subject("RCOS Password Reset")
            .to(&emails)
            .from(ctx.email_sender())
            .alternative(recovery_email.html(), recovery_email.plaintext())
            .build()
            .expect("Could not build email");



        unimplemented!()
    } else {
        form_page = form_page.error("Email Not Found");
        let page = ctx.render_in_page(&form_page, "Forgot Password").await;
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(page)
    }
}
