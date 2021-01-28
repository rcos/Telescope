use actix_web::{web::Form, HttpResponse};

use crate::{
    models::{emails::Email, recoveries::Recovery, users::User},
    templates::{
        emails::recovery_email::PasswordRecoveryEmail, forms::recovery::ForgotPasswordPage,
        Template
    },
    web::RequestContext,
};

use futures::prelude::*;

/// Form submitted by users to recovery service to set a new password.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordRecoveryForm {
    email: String,
}

/// The password recovery page.
#[get("/forgot")]
pub async fn forgot_page(ctx: RequestContext) -> HttpResponse {
    let form: Template = ForgotPasswordPage::new().as_template();
    let rendered: String = ctx.render_in_page(&form, "Forgot Password").await;
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
    let mut form_page: ForgotPasswordPage = ForgotPasswordPage::new().email(email);
    let database_result: Option<User> =
        Email::get_user_from_db_by_email(ctx.get_db_conn().await, email.to_string()).await;
    if let Some(target_user) = database_result {
        // get the user's emails.
        let emails: Vec<String> = target_user
            .get_emails_from_db(ctx.get_db_conn().await)
            .map(|emails: Vec<Email>| emails.into_iter().map(|e| e.email).collect())
            .await;

        // make a recovery record
        let recovery = Recovery::for_user(&target_user);

        // make recovery link
        let link = ctx
            .request()
            .uri()
            .authority()
            .map(|a| format!("https://{}/recover/{}", a.as_str(), recovery.recovery_id))
            // since lettre doesn't currently store the messages in a human readable
            // format in stub or file transport, we log the generated address here.
            .map(|url| {
                trace!("Generated recovery URL: {}", url);
                url
            })
            .expect("Could not make recovery URL.");

        // make the recovery email
        let recovery_email = PasswordRecoveryEmail::new(recovery.clone(), link);

        let mut email_builder = lettre_email::Email::builder().subject("RCOS Password Reset");

        // add all recipient emails.
        for e in emails {
            email_builder = email_builder.to(e);
        }

        let email = email_builder
            .from(ctx.email_sender())
            .alternative(
                ctx.render(&recovery_email.html()),
                ctx.render(&recovery_email.plaintext()),
            )
            .build()
            .expect("Could not build email");

        // send the email
        let email_result = ctx.send_mail(email).await;
        if email_result.is_err() {
            form_page =
                form_page.error("Could not send email. Please contact a server administrator.");
            return HttpResponse::InternalServerError()
                .body(ctx.render_in_page(&form_page.as_template(), "Error").await);
        }

        // store the recovery to the database.
        let db_res = recovery.store(ctx.get_db_conn().await).await;

        if let Err(err_msg) = db_res {
            form_page = form_page.error(err_msg);
            HttpResponse::InternalServerError()
                .body(ctx.render_in_page(&form_page.as_template(), "Error").await)
        } else {
            form_page.success = true;
            HttpResponse::Ok().body(
                ctx.render_in_page(&form_page.as_template(), "Email Sent")
                    .await,
            )
        }
    } else {
        form_page = form_page.error("Email Not Found");
        let page = ctx
            .render_in_page(&form_page.as_template(), "Forgot Password")
            .await;
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(page)
    }
}
