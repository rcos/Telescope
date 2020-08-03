use crate::web::api::User;
use crate::web::RequestContext;
use actix_web::{
    web::block,
    error::BlockingError
};
use diesel::result::Error;

// /// Create a new user and insert into database.
// pub async fn create_user(
//     req_ctx: &RequestContext,
//     name_arg: impl Into<String>,
//     password: &str
// ) -> Result<usize, BlockingError<Error>> {
//     use crate::schema::users::dsl::*;
//     let user = User::new(name_arg, password);
//     let db_conn = req_ctx.get_db_connection();
//     block(move || {
//         diesel::insert_into(users)
//             .values(&user)
//             .execute(&db_conn)
//     }).await
// }

/// Get a user by uuid.
pub async fn get_user(req_ctx: &RequestContext, uuid: &str) -> Option<User> {
    unimplemented!()
    // use crate::schema::users::dsl::*;
    // let db_conn = req_ctx.get_db_connection();
    // users.filter();
}