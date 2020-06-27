use crate::db::model::User;
use diesel::RunQueryDsl;
use crate::web::RequestContext;
use actix_web::web::block;
use actix_web::error::BlockingError;

/// Create a new user and insert into database.
pub async fn create_user(
    req_ctx: &RequestContext,
    name_arg: impl Into<String>,
    password: &str
) -> Result<usize, BlockingError<diesel::result::Error>> {
    use crate::schema::users::dsl::*;
    let user = User::new(name_arg, password);
    block(|| {
        let db_conn = req_ctx.get_db_connection();
        diesel::insert_into(users)
            .values(&user)
            .execute(&db_conn)
    }).await

}

/// Get a user by uuid.
pub async fn get_user(req_ctx: &RequestContext, uuid: &str) -> Option<User> {
    use crate::schema::users::dsl::*;
    let db_conn = req_ctx.get_db_connection();
    users.filter();
}