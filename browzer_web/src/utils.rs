//! This module contains various utilities used by the `browzer_web` like `HttpMethod` etc

pub mod thread_pool;

/// Enumeration of supported HTTP methods.
#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}
impl HttpMethod {
    /// Converts an `HttpMethod` enum value to its corresponding method string.
    ///
    /// # Returns
    ///
    /// A `String` representing the HTTP method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::utils::HttpMethod;
    ///
    /// let method = HttpMethod::GET;
    /// assert_eq!(method.to_string(), "GET".to_string());
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
        }
        .to_string()
    }
}

/// Enumeration of supported HTTP status codes.
#[derive(Debug, Clone)]
pub enum HttpStatusCode {
    OK,
    Created,
    Accepted,
    NoContent,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
}
impl HttpStatusCode {
    /// Converts an `HttpStatusCode` enum value to a tuple containing its corresponding reason phrase and status code.
    ///
    /// # Returns
    ///
    /// A tuple containing a `&str` representing the reason phrase and a `u16` representing the status code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::utils::HttpStatusCode;
    ///
    /// let status = HttpStatusCode::OK;
    /// assert_eq!(status.code(), ("OK", 200));
    /// ```
    pub fn code(&self) -> (&str, u16) {
        match self {
            HttpStatusCode::OK => ("OK", 200),
            HttpStatusCode::Created => ("Created", 201),
            HttpStatusCode::Accepted => ("Accepted", 202),
            HttpStatusCode::NoContent => ("NoContent", 204),
            HttpStatusCode::MovedPermanently => ("Moved Permanently", 301),
            HttpStatusCode::Found => ("Found", 302),
            HttpStatusCode::SeeOther => ("See Other", 303),
            HttpStatusCode::NotModified => ("Not Modified", 304),
            HttpStatusCode::BadRequest => ("Bad Request", 400),
            HttpStatusCode::Unauthorized => ("Unauthorized", 401),
            HttpStatusCode::Forbidden => ("Forbidden", 403),
            HttpStatusCode::NotFound => ("Not Found", 404),
            HttpStatusCode::MethodNotAllowed => ("Method Not Allowed", 405),
            HttpStatusCode::InternalServerError => ("Internal Server Error", 500),
            HttpStatusCode::NotImplemented => ("Not Implemented", 501),
            HttpStatusCode::BadGateway => ("Bad Gateway", 502),
            HttpStatusCode::ServiceUnavailable => ("Service Unavailable", 503),
        }
    }
}
