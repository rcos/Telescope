use crate::web::RequestContext;
use actix_web::web::{
    Path,
};
use uuid::Uuid;
use actix_web::HttpResponse;
use crate::templates::profile::Profile;
use crate::models::User;
use futures::future::OptionFuture;
use crate::templates::jumbotron::Jumbotron;

/// The service to display a user profile. The user is specified by the id in the
/// request path.
#[get("/profile/{uid}")]
pub async fn profile_service(ctx: RequestContext, user_id: Path<Uuid>) -> HttpResponse {
    let t_uid: Uuid = user_id.into_inner();
    let target = User::get_from_db_by_id(ctx.get_db_connection(), t_uid).await;

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

        let viewer = OptionFuture::from({
            ctx.identity()
                .identity()
                .map(|s| Uuid::parse_str(s.as_str()).ok())
                .flatten()
                .map(|v_uid| {
                    User::get_from_db_by_id(ctx.get_db_connection(), v_uid)
                })
        })
            .await
            .flatten();


        /*
        let mut profile = Profile {
            editable: false,
            name: user.name,
            picture: user.avi_location
        };


        if viewer.is_some() {
            let v = viewer.unwrap().await;
        }

         */
        unimplemented!()
    }
}