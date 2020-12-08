use actix_web::{web::Path, HttpResponse};

use uuid::Uuid;

use crate::{
    models::users::User,
    templates::{
        jumbotron,
        profile::Profile
    },
    web::RequestContext,
};

/// The service to display a user profile. The user is specified by the id in the
/// request path.
#[get("/profile/{uid}")]
pub async fn profile(ctx: RequestContext, Path(t_uid): Path<Uuid>) -> HttpResponse {
    let target = User::get_from_db_by_id(ctx.get_db_conn().await, t_uid).await;

    if target.is_none() {
        return HttpResponse::NotFound().body(
            jumbotron::rendered_page(&ctx, "User Not Found", "404", "User not found.").await,
        );
    } else {
        let user = target.unwrap();
        let page_title = format!("RCOS - {}", user.name.as_str());
        let profile = Profile::for_user(user, &ctx).await;
        let rendered = ctx.render_in_page(&profile, page_title).await;
        HttpResponse::Ok().body(rendered)
    }
}
