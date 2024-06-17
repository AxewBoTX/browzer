//! This module defines the `Request` struct and functionality related to handling HTTP requests.

// internal crate imports
use crate::{error, utils};

// standard library imports
use std::collections::HashMap;

/// Represents an HTTP request.
///
/// The `Request` struct contains all the information of an HTTP request, such as the HTTP method,
/// request path, HTTP version, request headers, and an optional request body.
///
/// # Fields
///
/// - `method` - The HTTP method of the request (e.g., GET, POST).
/// - `path` - The path of the request (e.g., "/index.html").
/// - `version` - The HTTP version used in the request (e.g., "HTTP/1.1").
/// - `headers` - A `HashMap` containing the request headers as key-value pairs.
/// - `body` - An optional string containing the body of the request.
/// - `cookies` - A `HashMap` containing cookies from the request
// ----- Request struct
#[derive(Debug)]
pub struct Request {
    pub method: utils::HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub cookies: HashMap<String, utils::Cookie>,
}
// default implementation for Request struct
impl Default for Request {
    fn default() -> Self {
        Request {
            method: utils::HttpMethod::GET,
            path: String::from("/"),
            version: String::from("HTTP/1.1"),
            headers: HashMap::new(),
            body: None,
            cookies: HashMap::new(),
        }
    }
}
impl Request {
    /// Creates a new `Request` instance from a vector of HTTP request strings.
    ///
    /// This function parses an HTTP request represented as a vector of strings and converts it into
    /// a `Request` struct. The vector should contain the request line (method, path, version),
    /// followed by headers, an empty line, and optionally a body.
    ///
    /// # Arguments
    ///
    /// - `input` - A reference to a vector of strings representing the HTTP request.
    ///
    /// # Returns
    ///
    /// - `Result<Request, error::RequestError>` - A result containing the `Request` struct if
    /// parsing is successful, or a `RequestError` if there is an error in parsing.
    ///
    /// # Errors
    ///
    /// - `RequestError::InvalidRequestLineError` - If the request line is malformed.
    /// - `RequestError::EmptyRequestError` - If the request is empty.
    pub fn new(input: &Vec<String>) -> Result<Request, error::RequestError> {
        let method;
        let path;
        let version;
        let mut headers = HashMap::new();

        // parse request method, path, and version from the first line of input string vector by
        // looping over the parts of the line
        match input.get(0) {
            Some(request_line) => {
                let parts: Vec<_> = request_line.split_whitespace().collect();
                if parts.len() >= 3 {
                    method = match parts[0] {
                        "GET" => utils::HttpMethod::GET,
                        "POST" => utils::HttpMethod::POST,
                        "PATCH" => utils::HttpMethod::PATCH,
                        "DELETE" => utils::HttpMethod::DELETE,
                        _ => utils::HttpMethod::GET,
                    };
                    path = parts[1].to_string();
                    version = parts[2].to_string();
                } else {
                    return Err(error::RequestError::InvalidRequestLineError(
                        request_line.to_string(),
                    ));
                }
            }
            None => return Err(error::RequestError::EmptyRequestError),
        }

        // parse headers into a string key-value pair hashmap by looping over the input string
        // vector elements and sperating key and value of headers by splitting at ":" and inserting
        // them into the `headers` hashmap
        let mut index = 1;
        while index < input.len() {
            let curr_line = &input[index];
            if curr_line.trim().is_empty() {
                break;
            }
            let parts: Vec<_> = curr_line.splitn(2, ":").map(|s| s.trim()).collect();
            if parts.len() == 2 {
                headers.insert(parts[0].to_string(), parts[1].to_string());
            }
            index += 1;
        }
        // parse body into a string by looping over the remaining input string vector elements and
        // joining them using the newline operator
        let body = if index + 1 < input.len() {
            Some(
                input[index + 1..]
                    .iter()
                    .map(|s| &**s) // NOTE: I have NO idea what is happening here
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
        } else {
            None
        };

        // parse cookies from `Cookie` header into the `cookies` field of the request
        let mut cookies = HashMap::new();
        if let Some(cookie_string) = headers.get("Cookie") {
            cookie_string.split(";").for_each(|string_cookie| {
                let mut cookie_parts = string_cookie.splitn(2, '=');
                if let (Some(name), Some(value)) = (cookie_parts.next(), cookie_parts.next()) {
                    cookies.insert(name.trim().to_string(), utils::Cookie::new(name, value));
                }
            });
        };

        // return the Request struct
        return Ok(Request {
            method,
            path,
            version,
            headers,
            body,
            cookies,
        });
    }
}
