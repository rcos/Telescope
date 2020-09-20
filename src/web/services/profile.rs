use crate::web::RequestContext;
use actix_web::web::{
    Path,
    block
};
use uuid::Uuid;
use actix_web::HttpResponse;
use crate::templates::profile::Profile;
use crate::models::User;

/// The service to display a user profile. The user is specified by the id in the
/// request path.
#[get("/profile/{uid}")]
pub async fn profile_service(ctx: RequestContext, user_id: Path<Uuid>) -> HttpResponse {
    use crate::schema::users;

    let uid: Uuid = user_id.into_inner();
    let identity = ctx.identity().identity();

    unimplemented!()

    /*
    let mut profile = Profile {
        name:
    };

    if let Some(viewer_id) = identity {

    }

     */
}