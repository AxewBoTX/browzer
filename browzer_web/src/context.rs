//! This module defines the `Context` struct, which represents the context of a web request.

// external crate imports
use serde_urlencoded;

// internal crate imports
use crate::{request, response, utils};

// standard library imports
use std::collections::HashMap;

/// Represents the context of a web request.
///
/// The `Context` struct contains information about the incoming request such as request details,
/// response to be sent back, parameters extracted from the request path, and query parameters and
/// much more.
///
/// # Fields
///
/// - `request` - The incoming request provided via the `Request` struct.
/// - `response` - The response to be sent back using the `Response` struct.
/// - `params` - A `HashMap` representing parameters extracted from the request path.
/// - `query_params` - A `HashMap` representing query parameters extracted from the request path.
///
/// # Examples
///
/// ```rust
/// let mut context = Context::new(Request::new());
/// let response = context.send_string(HttpStatusCode::OK, "Hello, World!");
/// ```
// ----- Context struct
#[derive(Debug)]
pub struct Context {
    pub request: request::Request,
    pub response: response::Response,
    pub params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

impl Context {
    /// Creates a new `Context` instance with the given request.
    ///
    /// # Arguments
    ///
    /// - `request` - A `Request` struct representing the incoming request.
    ///
    /// # Returns
    ///
    /// A new `Context` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let request = Request::new();
    /// let context = Context::new(request);
    /// ```
    pub fn new(request: request::Request) -> Context {
        Context {
            request,
            response: response::Response::default(),
            params: HashMap::new(),
            query_params: HashMap::new(),
        }
    }

    /// Constructs a response with the given status code and body content.
    ///
    /// # Arguments
    ///
    /// - `status_code` - A `HTTPStatusCode` specifying the status code of the response.
    /// - `input` - A `String` representing the body content of the response.
    ///
    /// # Returns
    ///
    /// A `Response` with the specified status code and body content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut context = Context::new(Request::new());
    /// let response = context.send_string(HttpStatusCode::OK, "Hello, World!");
    /// ```
    pub fn send_string(
        &mut self,
        status_code: utils::HttpStatusCode,
        input: &str,
    ) -> response::Response {
        let res = &mut self.response;
        res.status_code = status_code;
        res.body = input.to_string();
        res.clone()
    }

    /// Constructs a redirect response with the given status code and target route.
    ///
    /// # Arguments
    ///
    /// - `status_code` - A `HTTPStatusCode` specifying the status code of the response.
    /// - `route` - A `String` specifying the target route to redirect to.
    ///
    /// # Returns
    ///
    /// A `Response` with the specified status code and target route to redirect the user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut context = Context::new(Request::new());
    /// let response = context.redirect(HttpStatusCode::FOUND, "/home");
    /// ```
    pub fn redirect(
        &mut self,
        status_code: utils::HttpStatusCode,
        route: &str,
    ) -> response::Response {
        let res = &mut self.response;
        res.headers
            .insert("Location".to_string(), route.to_string());
        res.status_code = status_code;
        res.clone()
    }

    /// This method allows the user to read the form data from the request
    pub fn form_value(&mut self, key: &str) -> String {
        match self.request.headers.get("Content-Type") {
            Some(content_type) => content_type,
            None => return String::from(""),
        };
        match serde_urlencoded::from_str::<HashMap<String, String>>(match &self.request.body {
            Some(body) => match std::str::from_utf8(body.trim().as_bytes()) {
                Ok(body_str) => body_str.trim(),
                Err(_) => return String::from(""),
            },
            None => return String::from(""),
        }) {
            Ok(data) => {
                match data.get(key) {
                    Some(value) => {
                        return value.to_string();
                    }
                    None => {
                        return String::from("");
                    }
                };
            }
            Err(_) => return String::from(""),
        };
    }
}
