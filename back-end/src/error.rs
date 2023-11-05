use std::error::Error;

use actix_web::{
    http::{header::CONTENT_TYPE, StatusCode},
    HttpResponse, HttpResponseBuilder, ResponseError,
};
use handlebars::RenderError;

/// Custom MIME Type for telescope errors. Should only be used internally
/// as a signal value.
pub const MAGIC_ERROR_MIME: &str = "application/prs.magic.error+json";

/// All major errors that can occur while responding to a request.
#[derive(Debug, From, Error, Display, Serialize, Deserialize)]
pub enum PillError {
    #[display(fmt = "Page Not Found")]
    /// 404 - Page not found. Use [`MagicError::ResourceNotFound`] instead
    /// when possible, as it will have more info.
    PageNotFound,

    #[display(fmt = "{header}: {message}")]
    /// 404 - Resource Not Found.
    ResourceNotFound {
        /// The header of the jumbotron to be displayed.
        header: String,
        /// The message to display under the jumbotron.
        message: String,
    },

    #[display(fmt = "{header}: {message}")]
    /// Upstream server returned error. This is usually when adding users to the
    /// RCOS Discord.
    GatewayError {
        /// The header on the jumbotron to be displayed.
        header: String,
        /// The message on the jumbotron to be displayed.
        message: String,
    },

    #[from]
    #[display(fmt = "Error rendering handlebars template: {_0}")]
    /// An error in rendering a handlebars template. This will report as
    /// an internal server error.
    RenderingError(#[serde(with = "RenderErrorDef")] RenderError),

    #[display(fmt = "Internal future canceled")]
    /// An internal future was canceled unexpectedly. This will always report
    /// as an internal server error.
    FutureCanceled,

    #[error(ignore)]
    #[display(fmt = "Internal server error: {_0}")]
    /// There was an internal server error.
    InternalServerError(String),

    #[display(fmt = "Bad Request - {header}: {message}")]
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
    #[display(fmt = "Error interacting with GitHub API: {_0}")]
    /// Error interacting with GitHub's GraphQL API. This should generally
    /// report as an ISE.
    GitHubApiError(String),

    #[error(ignore)]
    #[display(fmt = "Error interacting with Discord API: {_0}")]
    /// Error interacting with the Discord API via Serenity. This should report
    /// as an ISE or a gateway error.
    SerenityError(String),

    // #[error(ignore)]
    // #[display(fmt = "Invalid form submission")]
    // /// The user submitted invalid data to a form. This should be reported as a
    // /// bad request and the form should be displayed for the user to try again.
    // /// The value here is the page to be displayed to the user.
    // InvalidForm(Page),
    #[display(fmt = "Request not properly authenticated")]
    /// An unauthenticated user is trying to access a page that requires
    /// authentication. Report as unauthorized and direct them to try again.
    NotAuthenticated,

    #[display(fmt = "Authenticated Request Forbidden")]
    /// An authenticated user tried to access a resource that they do not have
    /// sufficient permissions to access.
    Forbidden,
    // #[display(fmt = "MongoDB Error: {_0}")]
    // /// An error interacting with MongoDB.
    // MongoError(String),
}

// This may produce a warning in some IDEs because the `Display` trait
// is derived. You can safely ignore it.
impl ResponseError for PillError {
    // Override the default status code (500 - Internal Server Error) here.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest { .. } | Self::CsrfTokenMismatch => StatusCode::BAD_REQUEST,
            Self::ResourceNotFound { .. } | Self::PageNotFound | Self::CsrfTokenNotFound => {
                StatusCode::NOT_FOUND
            }
            Self::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            // Self::InvalidForm(_) => StatusCode::BAD_REQUEST,
            Self::NotAuthenticated => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::GatewayError { .. } => StatusCode::BAD_GATEWAY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // Override the default http response here.
    // Panic if the error cannot be serialized.
    fn error_response(&self) -> HttpResponse {
        // Firstly log the error, so we at least know what it was before
        // being serialized.
        // error!("Service generated error: {}", self);

        // Since we cannot render the html page here, we serialize
        // it to JSON and let the custom error handling middleware
        // render the HTTP page off of it later.
        let json_str: String =
            serde_json::to_string(self).expect("Could not serialize self to JSON.");

        // Create and return the response with the JSON and the custom
        // content type here.
        HttpResponseBuilder::new(self.status_code())
            .insert_header((CONTENT_TYPE, MAGIC_ERROR_MIME))
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
        let mut new: Self = Self::new(err.desc);
        new.column_no = err.column_no;
        new.line_no = err.line_no;
        new.template_name = err.template_name;
        new
    }
}