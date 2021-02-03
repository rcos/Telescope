//! Utility types and functions.

use actix_web::rt::blocking::BlockingError;
use std::error::Error;
use diesel::{
    r2d2::{PooledConnection, ConnectionManager},
    PgConnection
};


/// Database connection type.
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Handle a blocking error, usually from a database query. Return a string
/// representing what happened.
pub fn handle_blocking_err<E: Error>(err: BlockingError<E>, msg: impl Into<String>) -> String {
    match err {
        BlockingError::Canceled => error!("Blocking call canceled"),
        BlockingError::Error(e) => error!("Blocking call failed: {}", e),
    }
    msg.into()
}
