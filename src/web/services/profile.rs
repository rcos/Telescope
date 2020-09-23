use actix_web::{
    HttpResponse,
    web::{
        Path,
    }
};

use uuid::Uuid;

use crate::{
    web::RequestContext,
    templates::{
        profile::Profile,
        jumbotron::Jumbotron,
        page::Page
    },
    models::{
        User,
        markdown::render as md_render
    }
};

use futures::future::OptionFuture;
use futures::TryFutureExt;

/// The service to display a user profile. The user is specified by the id in the
/// request path.
#[get("/profile/{uid}")]
pub async fn profile_service(ctx: RequestContext, user_id: Path<Uuid>) -> HttpResponse {
    let t_uid: Uuid = user_id.into_inner();
    let target = User::get_from_db_by_id(ctx.get_db_connection().await, t_uid).await;

    if target.is_none() {
        return HttpResponse::NotFound()
            .body(Jumbotron::jumbotron_page(
                &ctx,
                "User Not Found",
                "404",
                "User not found."
            ));
    } else {
        let user = target.unwrap();

        let viewer = ctx.user_identity().await;

        // generate gravatar url

        // emails should always be non-empty on existing user.
        let emails = user.get_emails_from_db(ctx.get_db_connection().await).await;
        let gravatar_email: &str = emails.first().unwrap().email.as_str();
        let gravatar_hash = md5::compute(gravatar_email.trim().to_lowercase());
        let gravatar_base_url = "https://www.gravatar.com/avatar/";
        let gravatar_default_extention = "?d=identicon";
        let gravatar_url = format!("{}{:x}{}", gravatar_base_url, gravatar_hash, gravatar_default_extention);

        let profile = Profile {
            // page is editable if its your page or if you are a sysadmin.
            editable: viewer
                .map(|u| u.sysadmin || u.id == user.id)
                .unwrap_or(false),
            name: user.name.clone(),
            picture: user.avi_location.unwrap_or(gravatar_url),
            bio: md_render(user.bio.as_str())
        };

        let page = Page::new(
            format!("RCOS - {}", user.name),
            ctx.render(&profile),
            &ctx
        );

        HttpResponse::Ok().body(ctx.render(&page))
    }
}
