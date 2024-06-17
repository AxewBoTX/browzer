//! This module defines the `Response` struct used to represent HTTP responses in the web framework.
//! It includes functionality to create, manipulate, and convert responses to strings for sending over the network

// external crate imports
use chrono;

// internal crate imports
use crate::utils;

// standard library imports
use std::collections::HashMap;

/// Represents an HTTP response.
///
/// The `Response` struct holds information about the status code, headers, and body of an HTTP response.
///
/// # Fields
///
/// - `status_code` - An `HttpStatusCode` representing the status of the response.
/// - `headers` - A `HashMap` containing key-value pairs of header names and values.
/// - `body` - A `String` containing the body of the response.
/// - `cookies` - A `HashMap` containing cookies from the request
///
/// # Examples
///
/// ```rust
/// use browzer_web::response::Response;
/// use browzer_web::utils::HttpStatusCode;
/// use maplit::hashmap;
///
/// let response = Response {
///     status_code: HttpStatusCode::OK,
///     headers: hashmap! {
///         "Content-Type".to_string() => "text/html".to_string()
///     },
///     body: "<html><body>Hello, World!</body></html>".to_string(),
/// };
///
/// assert_eq!(response.status_code, HttpStatusCode::OK);
/// assert_eq!(response.headers.get("Content-Type").unwrap(), "text/html");
/// assert_eq!(response.body, "<html><body>Hello, World!</body></html>");
/// ```
// ----- Response struct
#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: utils::HttpStatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub cookies: HashMap<String, utils::Cookie>,
}

// default implementation for Response struct
impl Default for Response {
    fn default() -> Self {
        return Response {
            status_code: utils::HttpStatusCode::OK,
            headers: HashMap::new(),
            body: String::from(""),
            cookies: HashMap::new(),
        };
    }
}

impl Response {
    /// Creates a new `Response` instance.
    ///
    /// This function initializes a `Response` with a specified status code and body.
    ///
    /// # Arguments
    ///
    /// - `status_code` - An `HttpStatusCode` representing the status of the response.
    /// - `body` - A `String` containing the body of the response.
    ///
    /// # Returns
    ///
    /// - `Response` - A new instance of `Response`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::response::Response;
    /// use browzer_web::utils::HttpStatusCode;
    ///
    /// let response = Response::new(HttpStatusCode::OK, "Hello, World!".to_string());
    ///
    /// assert_eq!(response.status_code, HttpStatusCode::OK);
    /// assert!(response.headers.is_empty());
    /// assert_eq!(response.body, "Hello, World!");
    /// ```
    pub fn new(status_code: utils::HttpStatusCode, body: String) -> Response {
        return Response {
            status_code,
            headers: HashMap::new(),
            body,
            cookies: HashMap::new(),
        };
    }

    /// Converts the `Response` instance into a string formatted as an HTTP response.
    ///
    /// This function convert the `Response` struct into a string to be sent as bytes by setting the status_code
    /// number, status_code text, and content-length in the `Status Line`, setting headers
    /// to the response string by looping over `headers` field in the Response struct and looping
    /// over the `cookies` field in the Response struct, and then finally adding a blank line
    /// followed by the body of the response to the response string
    ///
    /// # Returns
    ///
    /// - A `String` representation of the HTTP response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::response::Response;
    /// use browzer_web::utils::HttpStatusCode;
    /// use maplit::hashmap;
    ///
    /// let mut cookies = hashmap! {
    ///     "session".to_string() => Cookie {
    ///         name: "session".to_string(),
    ///         value: "abc123".to_string(),
    ///         expires: Some(SystemTime::now() + Duration::new(15 * 60, 0)),
    ///         path: Some("/".to_string()),
    ///         domain: Some("example.com".to_string()),
    ///         secure: true,
    ///         http_only: true,
    ///         ..Default::default()
    ///     }
    /// };
    ///
    /// let response = Response {
    ///     status_code: HttpStatusCode::OK,
    ///     headers: hashmap! {
    ///         "Content-Type".to_string() => "text/html".to_string()
    ///     },
    ///     body: "<html><body>Hello, World!</body></html>".to_string(),
    /// };
    ///
    /// let response_string = response.to_string();
    ///
    /// assert!(response_string.contains("HTTP/1.1 200 OK"));
    /// assert!(response_string.contains("Content-Length: 39"));
    /// assert!(response_string.contains("Content-Type: text/html"));
    /// assert!(response_string.contains("<html><body>Hello, World!</body></html>"));
    /// assert!(response_string.contains("Set-Cookie: session=abc123; Path=/; Domain=example.com; Expires="));
    /// ```
    pub fn to_string(&self) -> String {
        let status_code = &self.status_code.code();
        let mut response = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n",
            status_code.1,
            status_code.0,
            &self.body.len(),
        );
        for (key, value) in &self.headers {
            response.push_str(&format! {"{}: {}\r\n",key,value});
        }

        // parse cookies hashmap and append it to the response string
        for cookie in self.cookies.values() {
            let mut cookie_string = format!("{}={}", cookie.name, cookie.value);

            if let Some(ref path) = cookie.path {
                cookie_string.push_str(&format!("; Path={}", path));
            }

            if let Some(ref domain) = cookie.domain {
                cookie_string.push_str(&format!("; Domain={}", domain));
            }

            if let Some(expires) = cookie.expires {
                let datetime = chrono::DateTime::<chrono::Utc>::from(expires);
                let formatted_time = datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
                cookie_string.push_str(&format!("; Expires={}", formatted_time));
            }

            if let Some(max_age) = cookie.max_age {
                cookie_string.push_str(&format!("; Max-Age={}", max_age));
            }

            if cookie.secure {
                cookie_string.push_str("; Secure");
            }

            if cookie.http_only {
                cookie_string.push_str("; HttpOnly");
            }

            response.push_str(&format!("Set-Cookie: {}\r\n", cookie_string));
        }

        response.push_str("\r\n");
        response.push_str(&self.body);
        return response;
    }
}
