use crate::{templates::forms::login, web::RequestContext};

use actix_web::{http::header, HttpResponse};

/// Log a user out.
///
/// Logout is GET only. Query variables (identical to those used in the login
/// format) may be used to indicate where to go after logout. If no query
/// variables are used, the default is the home page.
#[get("/logout")]
pub async fn logout_service(req_ctx: RequestContext) -> HttpResponse {
    req_ctx.identity().forget();
    let target = login::target_page(&req_ctx);
    HttpResponse::Found()
        .header(header::LOCATION, target)
        .finish()
}
