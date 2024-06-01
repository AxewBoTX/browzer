// external crate imports
use maplit::hashmap;

// internal crate imports
use crate::error::*;

// standard library imports
use std::collections::HashMap;

// HTTP Methods
#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}
impl HttpMethod {
    // converting enum value to method string
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

// ----- Request struct
#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
// default implementation for Request struct
impl Default for Request {
    fn default() -> Self {
        Request {
            method: HttpMethod::GET,
            path: String::from("/"),
            version: String::from("HTTP/1.1"),
            headers: hashmap! {},
            body: None,
        }
    }
}
impl Request {
    // convert a Http request string vector to `Request` struct
    pub fn new(input: &Vec<String>) -> Result<Request, RequestError> {
        let method;
        let path;
        let version;
        let mut headers = hashmap! {};

        // parse request method, path, and version from the first line of input string vector by
        // looping over the parts of the line
        match input.get(0) {
            Some(request_line) => {
                let parts: Vec<_> = request_line.split_whitespace().collect();
                if parts.len() >= 3 {
                    method = match parts[0] {
                        "GET" => HttpMethod::GET,
                        "POST" => HttpMethod::POST,
                        "PATCH" => HttpMethod::PATCH,
                        "DELETE" => HttpMethod::DELETE,
                        _ => HttpMethod::GET,
                    };
                    path = parts[1].to_string();
                    version = parts[2].to_string();
                } else {
                    return Err(RequestError::InvalidRequestLineError(
                        request_line.to_string(),
                    ));
                }
            }
            None => return Err(RequestError::EmptyRequestError),
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
                    .map(|s| &**s)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
        } else {
            None
        };

        // return the Request struct
        return Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        });
    }
}
