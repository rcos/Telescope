use crate::models::User;
use crate::{
    templates::{
        static_pages::index::LandingPage, static_pages::StaticPage, with_alert::WithAlert,
    },
    web::{DbConnection, RequestContext},
};
use actix_identity::Identity;
use actix_web::{
    web::{block, Form},
    Error, HttpResponse,
};
use std::collections::HashMap;
use uuid::Uuid;

const EMAIL_FIELD: &'static str = "email";
const PASSWORD_FIELD: &'static str = "pass";

/// Guarded to only post requests.
///
/// On successful login, set the secret secure identity cookie
/// to the user id and redirect the user to the page they are currently on.
///
/// On failure, return the landing page with an alert as to why they couldn't
/// login.
pub async fn login_service(
    req_ctx: RequestContext,
    login: Form<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let identity: &Identity = req_ctx.identity();
    if login.contains_key(EMAIL_FIELD) && login.contains_key(PASSWORD_FIELD) {
        let login_email_ref = login.get(EMAIL_FIELD).unwrap();
        let login_email: String = login_email_ref.clone();
        let login_password: String = login.get(PASSWORD_FIELD).unwrap().clone();
        let conn_pool_clone = req_ctx.clone_connection_pool();

        let mut db_res: Vec<(Uuid, String, String)> = block(move || {
            use crate::schema::{emails::dsl::*, users::dsl::*};
            use diesel::prelude::*;
            let conn: DbConnection = conn_pool_clone.get().unwrap();
            emails
                .inner_join(users)
                .filter(email.eq(login_email))
                .limit(1)
                .select((id, hashed_pwd, name))
                .load(&conn)
        })
        .await?;

        if db_res.is_empty() {
            let page = WithAlert::render_into_page(
                &req_ctx,
                LandingPage::PAGE_TITLE,
                "danger",
                format!("No user exists with email {}", login_email_ref),
                &LandingPage,
            );
            Ok(HttpResponse::NotFound().body(page))
        } else {
            let (user_id, hashed_pass, name) = db_res.pop().unwrap();
            let verified: bool =
                argon2::verify_encoded(hashed_pass.as_str(), login_password.as_bytes())
                    .map_err(|e| {
                        error!("Argon2 verification error {}", e);
                        e
                    })
                    .unwrap_or(false);
            if verified {
                identity.remember(User::format_uuid(user_id));
                Ok(HttpResponse::Ok().body(WithAlert::render_into_page(
                    &req_ctx,
                    LandingPage::PAGE_TITLE,
                    "success",
                    format!("Welcome {}!", name),
                    &LandingPage,
                )))
            } else {
                let page = WithAlert::render_into_page(
                    &req_ctx,
                    LandingPage::PAGE_TITLE,
                    "danger",
                    "Incorrect Password.",
                    &LandingPage,
                );
                Ok(HttpResponse::NotFound().body(page))
            }
        }
    } else {
        let page = WithAlert::render_into_page(
            &req_ctx,
            LandingPage::PAGE_TITLE,
            "danger",
            "Failed to login -- malformed request.",
            &LandingPage,
        );
        Ok(HttpResponse::BadRequest().body(page))
    }
}
