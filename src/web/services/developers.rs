use crate::templates::developers::DevelopersPage;
use crate::templates::Template;
use crate::{
    models::users::User,
    templates::{developers::UserThumbnail, static_pages::ise::ise},
    web::RequestContext,
    util::DbConnection,
};
use actix_web::HttpResponse;
use futures::future::join_all;

#[get("/developers")]
pub async fn developers_page(ctx: RequestContext) -> HttpResponse {
    // Try to get users from the database. Do error checking.
    let conn: DbConnection = ctx.get_db_conn().await;
    let database_result: Result<Vec<User>, String> = User::get_all_from_db(conn).await;
    if database_result.is_err() {
        return ise(&ctx).await;
    }

    // Get the thumbnails for each user.
    let thumbnail_futures = database_result
        .unwrap()
        .into_iter()
        .map(|u| async {
            let conn: DbConnection = ctx.get_db_conn().await;
            UserThumbnail::for_user(u, conn).await
        })
        .collect::<Vec<_>>();

    // Wait for all futures to resolve.
    let thumbnails: Vec<UserThumbnail> = join_all(thumbnail_futures).await;

    // Create and render template.
    let template: Template = DevelopersPage::new(thumbnails).template();
    let rendered: String = ctx
        .render_in_page(&template, DevelopersPage::PAGE_TITLE)
        .await;

    return HttpResponse::Ok().body(rendered);
}
