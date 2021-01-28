//! CRUD (Create, Read, Update, Delete) methods/traits for items in the
//! Database.

use diesel::{
    pg::Pg,
    Queryable,
    QueryResult,
    result::Error as DieselError
};
use crate::web::DbConnection;
use actix_web::{
    web::block,
    Responder,
    error::BlockingError,
    Error as ActixError
};

/// Telescope uses a Postgres Database.
type Db = Pg;

/// Types of error that can be result from trying to access the database.
pub enum AsyncDbError {
    /// Async future cancelled.
    BlockingCanceled,
    /// Record not found in database
    NotFound,
    /// Other issue with database
    Other(DieselError)
}

// impl Into<ActixError> for AsyncDbError {
//     fn into(self) -> ActixError {
//         match self {
//             AsyncDbError::BlockingCanceled => BlockingError::Canceled.into(),
//             AsyncDbError::Other(db_err) => BlockingError::Error(db_err).into(),
//             AsyncDbError::NotFound => unimplemented!()
//         }
//     }
// }

pub trait DbCreate {

}

// #[async_trait]
// pub trait DbRead: Sized + Send {
//     /// The type used to find the object in the database.
//     type Selector;
//
//     /// Read the target object from the database synchronously.
//     fn read_from_db_sync(selector: Self::Selector, db_conn: DbConnection) -> QueryResult<Self>;
//
//     /// Asynchronously get the object from the database using Actix blocking fn.
//     /// This just wraps a call to the sync function.
//     async fn get(selector: Self::Selector, db_conn: DbConnection) -> Result<Self, AsyncDbError> {
//         block::<_, Self, DieselError>(move || {
//             Self::read_from_db_sync(selector, db_conn)
//         }).await
//             .map_err(|e: BlockingError<DieselError>| match e {
//                 BlockingError::Canceled => AsyncDbError::BlockingCanceled,
//                 BlockingError::Error(db_err) => match db_err {
//                     DieselError::NotFound => AsyncDbError::NotFound,
//                     _ => AsyncDbError::Other(db_err)
//                 }
//             })
//     }
// }

pub trait DbUpdate {

}

pub trait DbDelete {

}