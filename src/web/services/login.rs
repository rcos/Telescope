use crate::{
    models::users::User,
    templates::{
        forms::{
            login,
            common::text_field
        },
        page,
        Template,
    },
    web::{
        api::rest::login::{
            login,
            LoginRequest
        },
        RequestContext,
    },
};

use actix_identity::Identity;

use actix_web::{http::header, web::Form, HttpResponse};

use uuid::Uuid;
use crate::web::api::rest::login::LoginError;

/// A request to the login form using a GET request. Sensitive user information
/// is not accepted when using GET.
#[get("/login")]
pub async fn login_get(req_ctx: RequestContext) -> HttpResponse {
    let target_page: String = login::target_page(&req_ctx);
    let identity: &Identity = req_ctx.identity();

    // check the identity.
    // if someone is already logged in then just redirect to the target page.
    let uid = identity
        .identity()
        .and_then(|s| Uuid::parse_str(s.as_str()).ok());

    if let Some(id) = uid {
        let conn = req_ctx.get_db_conn().await;
        let user = User::get_from_db_by_id(conn, id).await;
        // logged into to a valid user using the get request.
        if user.is_some() {
            return HttpResponse::Found()
                .header(header::LOCATION, target_page)
                .finish();
        }
    }
    // if the identity is empty or malformed (or if the user doesn't exist)
    // forget it and return the login form.
    identity.forget();

    let form: Template = login::new(&req_ctx);
    let login_page: Template = page::of(&req_ctx,"RCOS Login", &form).await;

    HttpResponse::Ok().body(req_ctx.render(&login_page))
}

/// The Login page and service.
#[post("/login")]
pub async fn login_post(req_ctx: RequestContext, form: Form<LoginRequest>) -> HttpResponse {
    let identity: &Identity = req_ctx.identity();
    let email: String = form.email.clone();
    let target_page: String = login::target_page(&req_ctx);
    let res: Result<User, LoginError> = login(&req_ctx, form.into_inner()).await;

    match res {
        Ok(user) => {
            // Modify the identity
            identity.remember(user.id_str());

            HttpResponse::Found()
                .header(header::LOCATION, target_page)
                .finish()
        },

        Err(e) => {
            let mut login_form: Template = login::new(&req_ctx);
            login_form[login::EMAIL][text_field::PREFILL_FIELD] = email.into();

            match e {
                LoginError::WrongPassword => {
                    login_form[login::PASSWORD][text_field::ERROR_FIELD] = "Incorrect password".into();
                },

                LoginError::EmailNotFound => {
                    login_form[login::EMAIL][text_field::ERROR_FIELD] = "Email not found".into();
                },
            }

            let page: Template = page::of(&req_ctx, "RCOS Login", &login_form).await;
            HttpResponse::Unauthorized().body(req_ctx.render(&page))
        }
    }
}
