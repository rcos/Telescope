//! Error handling.

use crate::templates::forms::FormTemplate;
use crate::templates::{jumbotron, page, Template};
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::Error as ActixError;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::http::StatusCode;
use actix_web::rt::blocking::BlockingError;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use graphql_client::Error as GraphQlError;
use handlebars::RenderError;
use reqwest::Error as ReqwestError;
use serde_json::Value;
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

    #[display(fmt = "{}: {}", header, message)]
    /// Upstream server returned error. This is usually when adding users to the
    /// RCOS Discord.
    GatewayError {
        /// The header on the jumbotron to be displayed.
        header: String,
        /// The message on the jumbotron to be displayed.
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
        /// Should the response status code be shown to the user?
        show_status_code: bool,
    },

    #[display(fmt = "Not Implemented")]
    /// Error to send when user accesses something that is not yet implemented.
    NotImplemented,

    #[display(fmt = "Could not extract IP address from HTTP request")]
    /// Error saving CSRF Token. This should report as an internal server error
    IpExtractionError,

    #[display(fmt = "Could not find CSRF token")]
    /// CSRF Token not found. This reports a Not Found status code but should
    /// usually be caught before reaching the user (if expected).
    CsrfTokenNotFound,

    #[display(fmt = "CSRF token mismatch")]
    /// The CSRF token provided by the HTTP request did not match the one
    /// generated by the server. This should be reported as a bad request.
    CsrfTokenMismatch,

    #[error(ignore)]
    #[display(fmt = "Error interacting with RCOS API: {}", _0)]
    /// Error interacting with RCOS central API.
    /// This should generally report as an ISE.
    RcosApiError(String),

    #[error(ignore)]
    #[display(fmt = "Error interacting with GitHub API: {}", _0)]
    /// Error interacting with GitHub's GraphQL API. This should generally
    /// report as an ISE.
    GitHubApiError(String),

    #[error(ignore)]
    #[display(fmt = "Error interacting with Discord API: {}", _0)]
    /// Error interacting with the Discord API via Serenity. This should report
    /// as an ISE or a gateway error.
    SerenityError(String),

    #[error(ignore)]
    #[display(fmt = "{} returned error(s) :{:?}", platform, errors)]
    /// A GraphQL API responded with errors. This should
    /// report as an internal server error.
    GraphQLError {
        /// The API platform
        platform: String,
        /// The errors that were returned.
        errors: Vec<GraphQlError>,
    },

    #[error(ignore)]
    #[display(fmt = "Invalid form submission")]
    /// The user submitted invalid data to a form. This should be reported as a
    /// bad request and the form should be displayed for the user to try again.
    /// The value here is the serde serialization of the form, since the [`FormTemplate`]
    /// type does not implement debug
    InvalidForm(Value),

    #[display(fmt = "Request not properly authenticated")]
    /// An unauthenticated user is trying to access a page that requires
    /// authentication. Report as unauthorized and direct them to try again.
    NotAuthenticated,

    #[display(fmt = "Authenticated Request Forbidden")]
    /// An authenticated user tried to access a resource that they do not have
    /// sufficient permissions to access.
    Forbidden,

    #[error(ignore)]
    #[display(fmt = "RPI CAS error: {}", _0)]
    /// Error sending to or receiving from the RPI CAS system.
    /// This should report as a Gateway error.
    RpiCasError(String),
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

    /// Convert a reqwest error from the RCOS API into a telescope error.
    pub fn rcos_api_error(err: ReqwestError) -> Self {
        error!("Error querying RCOS API: {}", err);
        Self::RcosApiError(err.to_string())
    }

    /// Convert a reqwest error from the GitHub API into a telescope error.
    pub fn github_api_error(err: ReqwestError) -> Self {
        error!("Error querying GitHub API: {}", err);
        Self::GitHubApiError(err.to_string())
    }

    /// Convert a Serenity error into a Telescope error.
    pub fn serenity_error(err: serenity::Error) -> Self {
        error!("Serenity Error: {}", err);
        Self::SerenityError(err.to_string())
    }

    /// Convert reqwest error from RPI CAS service into a Telescope error.
    pub fn rpi_cas_error(err: ReqwestError) -> Self {
        error!("Error querying RPI CAS endpoint: {}", err);
        TelescopeError::RpiCasError(err.to_string())
    }

    /// Serialize an invalid form to send back to the user.
    pub fn invalid_form(form: &FormTemplate) -> Self {
        // Convert the form to a JSON value.
        let value = serde_json::to_value(form)
            // Form should serialize without issue.
            .expect("Could not serialize form");

        // Construct and return the variant.
        return TelescopeError::InvalidForm(value);
    }

    /// Function that should only be used by the middleware to render a
    /// telescope error into an error page.
    pub async fn render_error_page(&self, req: &HttpRequest) -> Result<String, ActixError> {
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

            TelescopeError::GatewayError { header, message } => jumbotron::new(
                format!("{} - {}", status_code, header),
                format!("{} Please contact a coordinator or faculty advisor.", message)
            ),

            TelescopeError::FutureCanceled => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                "An internal future was canceled unexpectedly. Please try again. If you \
                keep seeing this error message, contact a coordinator and open an issue on the \
                Telescope GitHub repository.",
            ),

            TelescopeError::RenderingError(err) => jumbotron::new(
                format!("{} - Internal Server Template Error", status_code),
                format!(
                    "{}. Please create an issue on Telescope's GitHub and contact a \
                coordinator.",
                    err
                ),
            ),

            TelescopeError::BadRequest { header, message, show_status_code} => {
                jumbotron::new(
                    // Decide whether or not to show the status code.
                    show_status_code
                        // With the status code
                        .then(|| format!("{} - {}", status_code, header))
                        // Without the status code
                        .unwrap_or(header.clone()),
                    message)
            }

            TelescopeError::IpExtractionError => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                "Could not determine remote IP address of this request for CSRF purposes. \
                Please contact a coordinator and create a GitHub issue.",
            ),

            TelescopeError::CsrfTokenNotFound => jumbotron::new(
                format!("{} - CSRF Token Not Found", status_code),
                "Could not find the CSRF token for this request. Please try again. If this \
                error continues, please contact a coordinator and create a GitHub issue.",
            ),

            TelescopeError::CsrfTokenMismatch => jumbotron::new(
                format!("{} - Bad CSRF Token", status_code),
                "The CSRF token supplied to the server by this request does not match the \
                one the server generated for this identity provider for this IP. If you believe \
                this is in error, please contact a coordinator and file a GitHub issue.",
            ),

            TelescopeError::RcosApiError(err) => jumbotron::new(
                format!("{} - Internal API Query Error", status_code),
                format!(
                    "Could not query the central RCOS API. Please contact a coordinator and file a \
                    GitHub issue. Internal error description: {}", err),
            ),

            TelescopeError::GitHubApiError(err) => jumbotron::new(
                format!("{} - GitHub API V4 Query Error", status_code),
                format!("Could not query the GitHub API. Please contact a coordinator and \
                    file a GitHub issue on the Telescope repository. Internal error description: {}",
                    err),
            ),

            TelescopeError::SerenityError(err) => jumbotron::new(
                format!("{} - Discord Error", status_code),
                format!("Error interacting with the Discord API. Please contact a \
                    coordinator and file a GitHub issue if this error persists. Internal error \
                    description: {}", err)
            ),

            TelescopeError::RpiCasError(err) => jumbotron::new(
                format!("{} - RPI CAS Error", status_code),
                format!("Issue communicating with the RPI CAS service. Please try again. \
                If the error persists, please contact a coordinator and create an issue on the \
                Telescope GitHub. Internal Error: {}", err)
            ),

            TelescopeError::GraphQLError { platform, errors } => {
                // Map all errors to their `Display` formatting.
                let errs: Vec<String> = errors.iter().map(|e| format!("{}", e)).collect();

                jumbotron::new(
                    format!("{} - {} Error", status_code, platform),
                    format!("The {} returned at least one error. Please \
                    contact a coordinator and create an issue on the telescope GitHub. Internal error \
                    description(s): {:?}", platform, errs)
                )
            }

            TelescopeError::InternalServerError(message) => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                format!(
                    "Telescope had an internal server error. Please contact a \
                coordinator and file a GitHub issue. Error description: {}",
                    message
                ),
            ),

            TelescopeError::InvalidForm(form) => {
                // Render the form.
                // Start with a conversion.
                let form: FormTemplate = serde_json::from_value(form.clone())
                    // This should not fail.
                    .expect("Form serialization error.");

                // Render the form.
                let page_content: String = form.render()?;
                // Put it in a page.
                return page::with_content(req, form.page_title, page_content.as_str())
                    .await?
                    // Render Page
                    .render()
                    // Convert errors as necessary.
                    .map_err(ActixError::from);
            }

            TelescopeError::NotAuthenticated => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                "You need to sign in to access this page. If you are trying to create an \
                account, please restart. Otherwise please sign in. If you have logged in, and this \
                page is unexpected, please contact a coordinator and create a GitHub issue.",
            ),

            TelescopeError::Forbidden => jumbotron::new(
                format!("{} - {}", status_code, canonical_reason),
                "You do not have the necessary permissions to access this page. If you \
                think this is in error, please contact a coordinator or faculty advisor."
            ),
        };

        // Put jumbotron in a page and return the content.
        return page::of(req, "RCOS - Error", &inner_template)
            .await
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
            TelescopeError::CsrfTokenNotFound => StatusCode::NOT_FOUND,
            TelescopeError::CsrfTokenMismatch => StatusCode::BAD_REQUEST,
            TelescopeError::InvalidForm(_) => StatusCode::BAD_REQUEST,
            TelescopeError::NotAuthenticated => StatusCode::UNAUTHORIZED,
            TelescopeError::Forbidden => StatusCode::FORBIDDEN,
            TelescopeError::RpiCasError(_) => StatusCode::BAD_GATEWAY,
            TelescopeError::GatewayError { .. } => StatusCode::BAD_GATEWAY,
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
