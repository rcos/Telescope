//! Error handling.

use crate::templates::{jumbotron, page, Template};
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::Error as ActixError;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::http::StatusCode;
use actix_web::rt::blocking::BlockingError;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use handlebars::RenderError;
use lettre::file::error::Error as LettreFileError;
use lettre::smtp::error::Error as LettreSmtpError;
use lettre::smtp::response::Response as SmtpResponse;
use std::error::Error;
use std::fmt;

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
    RenderingError(#[serde(with = "RenderErrorDef")] RenderError),

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

    #[display(fmt = "Lettre File Error: {}", description)]
    #[error(ignore)]
    /// Error sending an email using lettre's file transport. This should report
    /// as an internal server error most of the time as file transport is used
    /// for debugging and logging.
    LettreFileError {
        #[serde(skip)]
        /// The lettre error that caused this. This gets stripped away on
        /// serialization.
        source: Option<LettreFileError>,
        /// A description of the cause.
        description: String,
    },

    #[display(fmt = "Lettre SMTP Error: {}", description)]
    #[error(ignore)]
    /// Error sending mail using lettre's SMTP transport. This should report as
    /// an internal server error when unexpected, but otherwise should
    /// be lowered to a form error and reported in the webpage.
    LettreSmtpError {
        #[serde(skip)]
        /// The lettre error that caused this. This gets stripped away during
        /// serialization.
        source: Option<LettreSmtpError>,
        /// The description of the error.
        description: String,
    },

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
            message: message.into(),
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
            message: message.into(),
        }
    }

    /// Function that should only be used by the middleware to render a
    /// telescope error into an error page.
    pub fn render_error_page(&self, req: &HttpRequest) -> Result<String, ActixError> {
        // Extract the path from the request for constructing the
        // page template later.
        let path = req.path();

        // Get the status code and canonical reason for this response.
        let status_code: u16 = self.status_code().as_u16();
        let canonical_reason: &'static str = self
            .status_code()
            .canonical_reason()
            .unwrap_or("Unknown Error");

        // Create an inner template depending on the error.
        let inner_template: Template = match self {
            TelescopeError::PageNotFound => jumbotron::new(
                format!("{} - Page Not Found", status_code),
                "We could not find the page you are looking for. If you think this is in \
                error, please reach out to a coordinator or make an issue on the Github repo.",
            ),

            TelescopeError::NotImplemented => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                "The telescope developers have not finished implementing this page. Please \
                contact a coordinator AND open a GitHub issue.",
            ),

            TelescopeError::ResourceNotFound { header, message } => {
                jumbotron::new(format!("{} - {}", status_code, header), message)
            }

            TelescopeError::FutureCanceled => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                "An internal future was canceled unexpectedly. Please try again. If you \
                keep seeing this error message, contact a coordinator and open an issue on the \
                Telescope GitHub repository.",
            ),

            TelescopeError::LettreFileError { description, .. } => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                format!(
                    "There was an error saving a server generated email to the local \
                filesystem. Please contact a coordinator and open a GitHub issue. Internal \
                error description: \"{}\"",
                    description
                ),
            ),

            TelescopeError::LettreSmtpError { description, .. } => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                format!(
                    "There was an error sending a server generated email via SMTP. \
                Please contact a coordinator and open a GitHub issue on the Telescope repository. \
                Internal error description: \"{}\"",
                    description
                ),
            ),

            TelescopeError::NegativeSmtpResponse(response) => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                format!(
                    "The internal SMTP client received a negative response. Please \
                contact a coordinator and create an issue on Telescope's GitHub repo. Error code \
                {}.",
                    response.code
                ),
            ),

            TelescopeError::RenderingError(err) => jumbotron::new(
                format!("{} - Internal Server Template Error", status_code),
                format!(
                    "{}. Please create an issue on Telescope's GitHub and contact a \
                coordinator.",
                    err
                ),
            ),

            TelescopeError::BadRequest { header, message } => {
                jumbotron::new(format!("{} - {}", status_code, header), message)
            }

            // If there is a variant without an error page implementation,
            // log an error message and render the unimplemented page.
            other => {
                return {
                    error!("{} does not have an error page implementation.", other);
                    TelescopeError::NotImplemented.render_error_page(req)
                }
            }
        };

        // Put jumbotron in a page and return the content.
        return page::of(path, "RCOS - Error", &inner_template)
            // Convert and handle jumbotron rendering errors.
            .map_err(ActixError::from)?
            // Render the page.
            .render()
            // Convert any error that occurs.
            .map_err(ActixError::from);
    }
}

impl<E> From<BlockingError<E>> for TelescopeError
where
    E: Into<TelescopeError> + fmt::Debug,
{
    fn from(error: BlockingError<E>) -> TelescopeError {
        match error {
            BlockingError::Canceled => TelescopeError::FutureCanceled,
            BlockingError::Error(e) => e.into(),
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
            TelescopeError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            TelescopeError::ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            TelescopeError::PageNotFound => StatusCode::NOT_FOUND,
            TelescopeError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // Override the default http response here.
    // Panic if the error cannot be serialized.
    fn error_response(&self) -> HttpResponse {
        // Firstly log the error, so we at least know what it was before
        // being serialized.
        error!("Service generated error: {}", self);

        // Since we cannot render the html page here, we serialize
        // it to JSON and let the custom error handling middleware
        // render the HTTP page off of it later.
        let json_str: String =
            serde_json::to_string(self).expect("Could not serialize self to JSON.");

        // Create and return the response with the JSON and the custom
        // content type here.
        HttpResponseBuilder::new(self.status_code())
            .set_header(CONTENT_TYPE, TELESCOPE_ERROR_MIME)
            .body(json_str)
    }
}

// Serde compatibility for remote types below.

#[derive(Serialize, Deserialize)]
#[serde(remote = "RenderError")]
/// Definition of foreign type that projects Serialization.
struct RenderErrorDef {
    /// Description of the error.
    desc: String,
    /// The name of the template that the error was in.
    template_name: Option<String>,
    /// The line that the error was on.
    line_no: Option<usize>,
    /// The column that the error was on.
    column_no: Option<usize>,

    #[serde(skip)]
    #[serde(getter = "Option::None")]
    /// Private field of remote struct. Skipped for serde.
    cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl From<RenderErrorDef> for RenderError {
    fn from(err: RenderErrorDef) -> Self {
        let mut new: RenderError = RenderError::new(err.desc);
        new.column_no = err.column_no;
        new.line_no = err.line_no;
        new.template_name = err.template_name;
        return new;
    }
}

impl From<LettreFileError> for TelescopeError {
    fn from(err: LettreFileError) -> Self {
        let description: String = format!("{}", err);

        return TelescopeError::LettreFileError {
            source: Some(err),
            description,
        };
    }
}

impl From<LettreSmtpError> for TelescopeError {
    fn from(err: LettreSmtpError) -> Self {
        let description: String = format!("{}", err);

        return TelescopeError::LettreSmtpError {
            source: Some(err),
            description,
        };
    }
}
