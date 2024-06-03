// internal crate imports
use crate::{request::*, response::*};

// ----- Context struct
#[derive(Debug)]
pub struct Context {
    pub request: Request,
    pub response: Response,
}

impl Context {
    // create a new Context object from request and response as inputs
    pub fn new(request: Request) -> Context {
        return Context {
            request,
            response: Response::default(),
        };
    }
    // send a string as response to a request
    pub fn send_string(&mut self, status_code: HttpStatusCode, input: &str) -> Response {
        let res = &mut self.response;
        res.status_code = status_code;
        res.body = input.to_string();
        return res.clone();
    }
}
