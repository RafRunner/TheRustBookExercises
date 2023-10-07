use anyhow::{anyhow, Result};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

mod body;
mod headers;
mod request;
mod response;

pub use body::HttpBody;
pub use headers::Headers;
pub use request::HttpRequest;
pub use response::HttpResponse;

pub enum HttpVersion {
    One,
    OnePointOne,
}

#[derive(EnumString, Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    Custom { name: String },
}

#[derive(Debug, PartialEq, Eq, Display)]
pub enum HttpStatus {
    // Informational
    Continue,
    SwitchingProtocols,
    Processing,

    // Success
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    // Redirecting
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    SwitchProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // Client errors
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // Server errors
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,

    Custom { code: u16, reason_phrase: String },
}

impl HttpVersion {
    pub fn build(string: &str) -> Result<Self> {
        match string {
            "HTTP/1.0" => Ok(Self::One),
            "HTTP/1.1" => Ok(Self::OnePointOne),
            _ => Err(anyhow!("Unsupported HTTP version")),
        }
    }
}

impl Debug for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::One => f.write_str("HTTP/1.0"),
            HttpVersion::OnePointOne => f.write_str("HTTP/1.1"),
        }
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl HttpMethod {
    fn new(method_name: &str) -> Self {
        Self::from_str(method_name).unwrap_or(HttpMethod::Custom {
            name: method_name.to_owned(),
        })
    }
}

impl HttpStatus {
    pub fn reason_phrase(&self) -> &str {
        self.props().1
    }

    pub fn code(&self) -> u16 {
        self.props().0
    }

    fn props(&self) -> (u16, &str) {
        match self {
            Self::Continue => (100, "Continue"),
            Self::SwitchingProtocols => (101, "Switching Protocols"),
            Self::Processing => (102, "Processing"),

            Self::Ok => (200, "OK"),
            Self::Created => (201, "Created"),
            Self::Accepted => (202, "Accepted"),
            Self::NonAuthoritativeInformation => (203, "Non-Authoritative Information"),
            Self::NoContent => (204, "No Content"),
            Self::ResetContent => (205, "Reset Content"),
            Self::PartialContent => (206, "Partial Content"),
            Self::MultiStatus => (207, "Multi-Status"),
            Self::AlreadyReported => (208, "Already Reported"),
            Self::IMUsed => (226, "IM Used"),

            Self::MultipleChoices => (300, "Multiple Choices"),
            Self::MovedPermanently => (301, "Moved Permanently"),
            Self::Found => (302, "Found"),
            Self::SeeOther => (303, "See Other"),
            Self::NotModified => (304, "Not Modified"),
            Self::UseProxy => (305, "Use Proxy"),
            Self::SwitchProxy => (306, "Switch Proxy"),
            Self::TemporaryRedirect => (307, "Temporary Redirect"),
            Self::PermanentRedirect => (308, "Permanent Redirect"),

            Self::BadRequest => (400, "Bad Request"),
            Self::Unauthorized => (401, "Unauthorized"),
            Self::PaymentRequired => (402, "Payment Required"),
            Self::Forbidden => (403, "Forbidden"),
            Self::NotFound => (404, "Not Found"),
            Self::MethodNotAllowed => (405, "Method Not Allowed"),
            Self::NotAcceptable => (406, "Not Acceptable"),
            Self::ProxyAuthenticationRequired => (407, "Proxy Authentication Required"),
            Self::RequestTimeout => (408, "Request Timeout"),
            Self::Conflict => (409, "Conflict"),
            Self::Gone => (410, "Gone"),
            Self::LengthRequired => (411, "Length Required"),
            Self::PreconditionFailed => (412, "Precondition Failed"),
            Self::PayloadTooLarge => (413, "Payload Too Large"),
            Self::URITooLong => (414, "URI Too Long"),
            Self::UnsupportedMediaType => (415, "Unsupported Media Type"),
            Self::RangeNotSatisfiable => (416, "Range Not Satisfiable"),
            Self::ExpectationFailed => (417, "Expectation Failed"),
            Self::ImATeapot => (418, "I'm a Teapot"),
            Self::MisdirectedRequest => (421, "Misdirected Request"),
            Self::UnprocessableEntity => (422, "Unprocessable Entity"),
            Self::Locked => (423, "Locked"),
            Self::FailedDependency => (424, "Failed Dependency"),
            Self::TooEarly => (425, "Too Early"),
            Self::UpgradeRequired => (426, "Upgrade Required"),
            Self::PreconditionRequired => (428, "Precondition Required"),
            Self::TooManyRequests => (429, "Too Many Requests"),
            Self::RequestHeaderFieldsTooLarge => (431, "Request Header Fields Too Large"),
            Self::UnavailableForLegalReasons => (451, "Unavailable For Legal Reasons"),

            Self::InternalServerError => (500, "Internal Server Error"),
            Self::NotImplemented => (501, "Not Implemented"),
            Self::BadGateway => (502, "Bad Gateway"),
            Self::ServiceUnavailable => (503, "Service Unavailable"),
            Self::GatewayTimeout => (504, "Gateway Timeout"),
            Self::HTTPVersionNotSupported => (505, "HTTP Version Not Supported"),
            Self::VariantAlsoNegotiates => (506, "Variant Also Negotiates"),
            Self::InsufficientStorage => (507, "Insufficient Storage"),
            Self::LoopDetected => (508, "Loop Detected"),
            Self::NotExtended => (510, "Not Extended"),
            Self::NetworkAuthenticationRequired => (511, "Network Authentication Required"),

            Self::Custom {
                code,
                reason_phrase,
            } => (*code, reason_phrase),
        }
    }
}
