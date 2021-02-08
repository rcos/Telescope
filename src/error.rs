//! Error handling.

use handlebars::RenderError;
use actix_web::rt::blocking::BlockingError;
use std::fmt;
use std::error::Error;
use lettre::file::error::Error as LettreFileError;
use lettre::smtp::error::Error as LettreSmtpError;
use lettre::smtp::response::Response as SmtpResponse;
use actix_web::{ResponseError, HttpResponse, HttpRequest};
use actix_web::http::StatusCode;
use actix_web::error::Error as ActixError;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::dev::{HttpResponseBuilder, ServiceResponse};
use serde::__private::Formatter;

/// Custom MIME Type for telescope errors. Should only be used internally
/// as a signal value.
pub const TELESCOPE_ERROR_MIME: &'static str = "application/prs.telescope.error+json";

/// All major errors that can occur while responding to a request.
#[derive(Debug, From, Error, Display, Serialize, Deserialize)]
pub enum TelescopeError {
    #[display(fmt = "Page Not Found")]
    /// 404 - Page not found. Use [`TelescopeError::ResourceNotFound`] instead
    /// when possible, as it will have more info.
    PageNotFound,

    #[display(fmt = "{}: {}", header, message)]
    /// 404 - Resource Not Found.
    ResourceNotFound {
        /// The header of the jumbotron to be displayed.
        header: String,
        /// The message to display under the jumbotron.
        message: String,
    },

    #[from]
    #[display(fmt = "Error rendering handlebars template: {}", _0)]
    /// An error in rendering a handlebars template. This will report as
    /// an internal server error.
    RenderingError(SimplifiedHandlebarsError),

    #[display(fmt = "Internal future canceled")]
    /// An internal future was canceled unexpectedly. This will always report
    /// as an internal server error.
    FutureCanceled,

    #[error(ignore)]
    #[display(fmt = "Internal server error: {}", _0)]
    /// There was an internal server error.
    InternalServerError(String),

    #[display(fmt = "Bad Request - {}: {}", header, message)]
    /// The request was malformed.
    BadRequest {
        /// The header of the jumbotron to be displayed.
        header: String,
        /// The error message to be displayed under the jumbotron.
        message: String,
    },

    #[from]
    #[display(fmt = "Lettre File Error: {}", _0)]
    /// Error sending an email using lettre's file transport. This should report
    /// as an internal server error most of the time as it is used for debugging
    /// and logging.
    LettreFileError(LettreFileError),

    #[from]
    #[display(fmt = "Lettre SMTP Error: {}", _0)]
    /// Error sending mail using lettre's SMTP transport. This should report as
    /// an internal server error when unexpected, but otherwise should
    /// be lowered to a form error and reported in the webpage.
    LettreSmtpError(LettreSmtpError),

    #[error(ignore)]
    #[display(fmt = "Negative SMTP response: {} - {:?}", "_0.code", "_0.message")]
    /// A negative response from the SMTP server, indicating a failure to
    /// authenticate or send an email. This should be reported as an internal
    /// server error where necessary but otherwise can be lowered to a form
    /// error.
    NegativeSmtpResponse(SmtpResponse),

    #[display(fmt = "Not Implemented")]
    /// Error to send when user accesses something that is not yet implemented.
    NotImplemented,
}

impl TelescopeError {
    /// Create a resource not found error with converted fields.
    pub fn resource_not_found(header: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ResourceNotFound {
            header: header.into(),
            message: message.into()
        }
    }

    /// Construct an Internal Server Error and convert the message.
    pub fn ise(message: impl Into<String>) -> Self {
        Self::InternalServerError(message.into())
    }

    /// Construct a Bad Request error and convert the fields.
    pub fn bad_request(header: impl Into<String>, message: impl Into<String>) -> Self {
        Self::BadRequest {
            header: header.into(),
            message: message.into()
        }
    }

    /// Function that should only be used by the middleware to render a
    /// telescope error into an error page.
    pub fn render_error_page(&self, req_path: String) -> Result<ServiceResponse, ActixError> {
        unimplemented!()
    }
}

impl<E> From<BlockingError<E>> for TelescopeError
where E: Into<TelescopeError> + fmt::Debug {
    fn from(error: BlockingError<E>) -> TelescopeError {
        match error {
            BlockingError::Canceled => TelescopeError::FutureCanceled,
            BlockingError::Error(e) => e.into()
        }
    }
}

impl From<SmtpResponse> for TelescopeError {
    /// Convert the SMTP response. Panic if it is not negative.
    fn from(res: SmtpResponse) -> Self {
        if res.is_positive() {
            panic!("Cannot construct error from positive SMTP response.");
        }
        TelescopeError::NegativeSmtpResponse(res)
    }
}

// This may produce a warning in some IDEs because the `Display` trait
// is derived. You can safely ignore it.
impl ResponseError for TelescopeError {
    // Override the default status code (500 - Internal Server Error) here.
    fn status_code(&self) -> StatusCode {
        match self {
            TelescopeError::BadRequest {..} => StatusCode::BAD_REQUEST,
            TelescopeError::ResourceNotFound {..} => StatusCode::NOT_FOUND,
            TelescopeError::PageNotFound => StatusCode::NOT_FOUND,
            TelescopeError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // Override the default http response here.
    // Panic if the error cannot be serialized.
    fn error_response(&self) -> HttpResponse {
        // Since we cannot render the html page here, we serialize
        // it to JSON and let the custom error handling middleware
        // render the HTTP page off of it later.
        let json_str: String = serde_json::to_string(self)
            .expect("Could not serialize self to JSON.");

        // Create and return the response with the JSON and the custom
        // content type here.
        HttpResponseBuilder::new(self.status_code())
            .set_header(CONTENT_TYPE, TELESCOPE_ERROR_MIME)
            .body(json_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
/// A simplified handlebars rendering error that can be serialized.
/// This struct is separate from the variant used above to make the
/// implementation easier, since the traits on [`TelescopeError`]
/// can be derived in most cases.
pub struct SimplifiedHandlebarsError {
    /// Description of the error.
    desc: String,
    /// The name of the template that the error was in.
    template_name: Option<String>,
    /// The line that the error was on.
    line_no: Option<usize>,
    /// The column that the error was on.
    column_no: Option<usize>
}

impl From<RenderError> for SimplifiedHandlebarsError {
    fn from(err: RenderError) -> Self {
        // Destructure original error.
        let RenderError {
            desc,
            template_name,
            line_no,
            column_no,
            ..
        } = err;

        // Restructure into simplified error.
        Self {desc, template_name, line_no, column_no}
    }
}

impl fmt::Display for SimplifiedHandlebarsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Display the description
        write!(f, "A handlebars rendering error occurred: {}", self.desc)?;

        // Check for a template name.
        if let Some(name) = self.template_name.as_ref() {
            write!(f, " in template {}", name)?;
            // Check for a line number.
            if let Some(line) = self.line_no {
                write!(f, " at line {}", line)?;

                // Check for a column number.
                if let Some(col) = self.column_no {
                    write!(f, ", column {}", col)?;
                }
            }
        }

        // Return Ok.
        Ok(())
    }
}

impl From<RenderError> for TelescopeError {
    fn from(err: RenderError) -> Self {
        // Convert the error and construct the variant.
        Self::RenderingError(SimplifiedHandlebarsError::from(err))
    }
}


