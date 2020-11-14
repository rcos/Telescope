use actix_web::rt::blocking::BlockingError;
use std::error::Error;

/// Handle a blocking error, usually from a database query. Return a string
/// representing what happened.
pub fn handle_blocking_err<E: Error>(err: BlockingError<E>, msg: impl Into<String>) -> String {
    match err {
        BlockingError::Canceled => error!("Blocking call canceled"),
        BlockingError::Error(e) => error!("Blocking call failed: {}", e),
    }
    msg.into()
}
