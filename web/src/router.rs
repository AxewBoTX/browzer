// external crate imports
use maplit::hashmap;
// internal crate imports
use crate::{context::*, request::*, response::*};
// standard library imports
use std::{collections::HashMap, fmt::Debug};

// ----- RouteHandler struct
pub struct RouteHandler {
    pub handler_func: Box<dyn Fn(Context) -> Response + 'static + Send + Sync>,
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
        F: Fn(Context) -> Response + 'static + Send + Sync,
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
    // handle response generation from request by first getting all the user-registered routes
    // which match the request's path(it will be hashmap) from `routes` hashmap, then using that
    // hashmap to get the route which matches request's method and then finaly using that route's
    // handler function to generate the response for the request by providing a new `Context` with
    // the request as input to the handler function
    pub fn handle_request(&self, request: Request) -> Response {
        match self.routes.get(&request.path) {
            Some(path_map) => match path_map.get(&request.method.to_string()) {
                Some(route_handler) => {
                    // the request path, method `exactly` matches a registered route path, method
                    return (route_handler.handler_func)(Context::new(request));
                }
                None => {
                    // the request path `exactly` matches a registered route path but the method is
                    // different
                    return Response::new(
                        HttpStatusCode::MethodNotAllowed,
                        format!("{}", HttpStatusCode::MethodNotAllowed.code().0).to_string(),
                    );
                }
            },
            // the request path does not `exactly` match a registered route path
            None => {
                for (route_path, method_map) in &self.routes {
                    match WebRouter::match_dynamic_route(
                        request.path.to_string(),
                        route_path.to_string(),
                    ) {
                        Some(params) => match method_map.get(&request.method.to_string()) {
                            Some(route_handler) => {
                                let mut context = Context::new(request);
                                context.params = params;
                                // the request path matches a registered dynamic route path pattern
                                // with provided parameters
                                return (route_handler.handler_func)(context);
                            }
                            None => {}
                        },
                        None => {}
                    }
                }
                // the request path neither `exactly` matches any registered route,
                // nor matches with any registered dynamic route path pattern
                return Response::new(
                    HttpStatusCode::NotFound,
                    format!("{}", HttpStatusCode::NotFound.code().0).to_string(),
                );
            }
        }
    }
    // match request path with registered dynamic route path by first converting both the paths into
    // vectors by splitting them at  `/` (backslashes), then comparing length of these vectors to
    // ensure they are both of same size, then we just `zip` these two vectors into 1 single
    // vector, with format `(request_path_part,route_path_part)`, then we loop over this vector and
    // check if the `route_path_part` of any item starts with ":" if it does, that mean that this
    // registered route is a dynamic route, so we store the corresponding `request_path_part` into
    // the params HashMap which is then returned after the for loop ends, and if the `route_path_part`
    // does not start with ":", that means it's a normal route and both the parts should be equal,
    // if they aren't, then we just return None,
    fn match_dynamic_route(
        request_path: String,
        route_path: String,
    ) -> Option<HashMap<String, String>> {
        let mut params: HashMap<String, String> = hashmap! {};

        let request_path_parts: Vec<&str> = request_path.split('/').collect();
        let route_path_parts: Vec<&str> = route_path.split('/').collect();

        if route_path_parts.len() != request_path_parts.len() {
            return None;
        }

        for (request_path_part, route_path_part) in
            request_path_parts.iter().zip(route_path_parts.iter())
        {
            if route_path_part.starts_with(':') {
                let param_name = &route_path_part[1..];
                params.insert(param_name.to_string(), request_path_part.to_string());
            } else if request_path_part != route_path_part {
                return None;
            }
        }
        Some(params)
    }
}
