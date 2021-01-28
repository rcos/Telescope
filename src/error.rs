//! Error handling.

use diesel::r2d2::PoolError;
use handlebars::RenderError;
use actix_web::rt::blocking::BlockingError;
use std::fmt;
use std::error::Error;

/// All major errors that can occur while responding to a request.
#[derive(Debug, From, Error)]
pub enum TelescopeError {
    /// 404 - Page not found. Use [`TelescopeError::ResourceNotFound`] instead
    /// when possible, as it will have more info.
    PageNotFound,

    /// 404 - Resource Not Found.
    ResourceNotFound {
        /// The header of the jumbotron to be displayed.
        header: String,
        /// The message to display under the jumbotron.
        message: String
    },

    #[from]
    /// An error in rendering a handlebars template. This will report as
    /// an internal server error.
    RenderingError(RenderError),

    #[from]
    /// Error retrieving connection to database from the database connection
    /// pool. This will always report as an internal server error.
    DbConnectionError(PoolError),

    /// An internal future was canceled unexpectedly. This will always report
    /// as an internal server error.
    FutureCanceled,
}

// impl TelescopeError {
//     /// Make a telescope error from a blocking error. This is a
//     /// function alias to the convert implementation.
//     pub fn from_blocking<E: Into<TelescopeError> + fmt::Debug>(e: BlockingError<E>) -> Self {
//         Self::from(e)
//     }
// }

impl<E> From<BlockingError<E>> for TelescopeError
where E: Into<TelescopeError> + fmt::Debug {
    fn from(error: BlockingError<E>) -> TelescopeError {
        match error {
            BlockingError::Canceled => TelescopeError::FutureCanceled,
            BlockingError::Error(e) => e.into()
        }
    }
}

impl fmt::Display for TelescopeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
