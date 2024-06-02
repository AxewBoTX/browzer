// external crate imports
use maplit::hashmap;

// standard library imports
use std::collections::HashMap;

// HTTP Response Status Codes
#[derive(Debug)]
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
    // HttpStatusCode function implementation for coverting Enum value to an unsigned integer
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

// ----- Response struct
#[derive(Debug)]
pub struct Response {
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

// default implementation for Response struct
impl Default for Response {
    fn default() -> Self {
        return Response {
            status_code: HttpStatusCode::OK,
            headers: hashmap! {},
            body: String::from(""),
        };
    }
}

impl Response {
    // creating a new `Response` struct using status_code and response body as input
    pub fn new(status_code: HttpStatusCode, body: String) -> Response {
        return Response {
            status_code,
            headers: hashmap! {},
            body,
        };
    }
    // convert the `Response` struct into a string to be sent as bytes by setting the status_code
    // number, status_code text, and content-length in the `Status Line`, setting headers
    // to the response string by looping over `headers` field in the Response struct, and then
    // finally adding a blank line followed by the body of the response to the response string
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
        response.push_str("\r\n");
        response.push_str(&self.body);
        return response;
    }
}
