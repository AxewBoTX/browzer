//! This module contains various utilities used by the `browzer_web` like `HttpMethod` etc

pub mod thread_pool;

// internal crate imports
use crate::error;

/// Formats the route or request path string by slashes
///
/// If there is a route defined as `/menu/items/`, a person would probably not want to add the
/// slash at the end everytime they are visiting this path, so this function removes the slashes at
/// the end from such paths making it easier and simpler for both the end user and developer
///
/// # Arguments
/// - `path` - A `String` representing the path to be formatted
///
/// # Returns
/// - `Result<String, WebRouterError>` - A result containing a `String` representing the formatted
/// path if it was successfully formatted or a `WebRouterError` if there is an error in formatting
/// the path.
///
/// # Examples
///
/// ```rust
/// assert_eq!(format_path_by_slashes("/menu/items/".to_string()), Ok("/menu/items".to_string()));
/// assert_eq!(format_path_by_slashes("/users/get_user".to_string()), Ok("/users/get_user".to_string()));
/// assert_eq!(format_path_by_slashes("/users/axew/?pass=\"some_pass\"".to_string()), Ok("/users/axew?pass=\"some_pass\"".to_string()));
/// assert_eq!(format_path_by_slashes("/".to_string()), Ok("/".to_string()));
/// ```
pub fn format_path_by_slashes(mut path: String) -> Result<String, error::WebRouterError> {
    if path.trim().len() == 0 && path.trim() == "" {
        path = "/".to_string();
    }
    match path.chars().nth(path.len() - 1) {
        Some(last_char) => {
            if last_char == '/' {
                path.pop();
            }
        }
        None => {
            return Err(error::WebRouterError::PathFormatError(
                "Failed to format path by slashes".to_string(),
            ));
        }
    }
    path = path.replace("/?", "?");
    return Ok(path);
}

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
