// external crate imports
use maplit::hashmap;
// internal crate imports
use crate::{request::*, response::*};
// standard library imports
use std::{collections::HashMap, fmt::Debug};

// ----- RouteHandler struct
pub struct RouteHandler {
    pub handler_func: Box<dyn Fn(Request) -> Response + 'static + Send + Sync>,
}
impl Debug for RouteHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouterHandler").finish()
    }
}
impl RouteHandler {
    // create a new RouteHandler using a handler closure function
    pub fn new<F>(handler: F) -> RouteHandler
    where
        F: Fn(Request) -> Response + 'static + Send + Sync,
    {
        return RouteHandler {
            handler_func: Box::new(handler),
        };
    }
}

// ----- WebRouter struct
#[derive(Debug)]
pub struct WebRouter {
    // HashMap< --path-- ,HashMap< --method-- ,RouteHandler>>
    pub routes: HashMap<String, HashMap<String, RouteHandler>>,
}

impl WebRouter {
    // create a new WebRouter with a completely empty route hashmap
    pub fn new() -> WebRouter {
        return WebRouter {
            routes: hashmap! {},
        };
    }
    // add a new route to the `routes` hashmap using route path, method and route handler as input
    pub fn add(&mut self, path: String, method: HttpMethod, handler: RouteHandler) {
        self.routes
            .entry(path.to_string())
            .or_insert_with(HashMap::new)
            .insert(method.to_string(), handler);
    }
}
