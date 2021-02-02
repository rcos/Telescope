use crate::templates::developers::DevelopersPage;
use crate::templates::{
    Template,
    page
};
use crate::{
    models::users::User,
    templates::{developers::UserThumbnail},
    web::RequestContext,
    util::DbConnection,
};
use actix_web::{HttpResponse, Responder};
use crate::error::TelescopeError;
use futures::future::try_join_all;

#[get("/developers")]
pub async fn developers_page(ctx: RequestContext) -> Result<Template, TelescopeError> {
    // Try to get users from the database. Do error checking.
    let all_users: Vec<User> = User::get_all_from_db().await?;

    // Get the thumbnails for each user.
    let thumbnail_futures = all_users
        .into_iter()
        .map(UserThumbnail::for_user)
        .collect::<Vec<_>>();

    // Wait for all futures to resolve.
    let thumbnails: Vec<UserThumbnail> = try_join_all(thumbnail_futures).await?;

    // Create and render template.
    let template: Template = DevelopersPage::new(thumbnails).template();
    page::of(&ctx, DevelopersPage::PAGE_TITLE, &template).await
}
