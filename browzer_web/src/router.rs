//! This module provides the routing functionality for the web framework. It defines the `RouteHandler` and `WebRouter` structs, allowing for the registration and handling of routes in a web application.

// external crate imports
use maplit::hashmap;
// internal crate imports
use crate::{context, request, response, utils};
// standard library imports
use std::{collections::HashMap, fmt::Debug};

/// Represents a route handler function.
///
/// The `RouteHandler` struct wraps a closure that processes a `Context` and returns a `Response`.
///
/// # Fields
///
/// - `handler_func` - A boxed closure that takes a `Context` and returns a `Response`.
///
/// # Examples
///
/// ```rust
/// use browzer_web::{context::Context, response::Response, router::RouteHandler, utils::HttpStatusCode};
///
/// let handler = RouteHandler::new(|ctx: Context| {
///     Response::new(HttpStatusCode::OK, "Hello, World!".to_string())
/// });
/// ```
// ----- RouteHandler struct
pub struct RouteHandler {
    pub handler_func: Box<dyn Fn(context::Context) -> response::Response + 'static + Send + Sync>,
}
impl Debug for RouteHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouterHandler").finish()
    }
}
impl RouteHandler {
    /// Creates a new `RouteHandler`.
    ///
    /// This function initializes a `RouteHandler` with a given closure.
    ///
    /// # Arguments
    ///
    /// - `handler` - A closure that takes a `Context` and returns a `Response`.
    ///
    /// # Returns
    ///
    /// - `RouteHandler` - A new instance of `RouteHandler`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::{context::Context, response::Response, router::RouteHandler, utils::HttpStatusCode};
    ///
    /// let handler = RouteHandler::new(|ctx: Context| {
    ///     Response::new(HttpStatusCode::OK, "Hello, World!".to_string())
    /// });
    /// ```
    pub fn new<F>(handler: F) -> RouteHandler
    where
        F: Fn(context::Context) -> response::Response + 'static + Send + Sync,
    {
        return RouteHandler {
            handler_func: Box::new(handler),
        };
    }
}

/// Manages the routing logic for the web framework.
///
/// The `WebRouter` struct holds the registered routes and matches incoming requests to the appropriate route handler.
///
/// # Fields
///
/// - `routes` - A `HashMap` mapping route paths to another `HashMap` of HTTP methods and their corresponding `RouteHandler`.
///
/// # Examples
///
/// ```rust
/// use browzer_web::{context::Context, response::Response, router::WebRouter, utils::HttpMethod, utils::HttpStatusCode};
/// use maplit::hashmap;
///
/// let mut router = WebRouter::new();
/// router.add("/".to_string(), HttpMethod::GET, RouteHandler::new(|ctx: Context| {
///     Response::new(HttpStatusCode::OK, "Hello, World!".to_string())
/// }));
/// ```
// ----- WebRouter struct
#[derive(Debug)]
pub struct WebRouter {
    // HashMap< --path-- ,HashMap< --method-- ,RouteHandler>>
    pub routes: HashMap<String, HashMap<String, RouteHandler>>,
}

impl WebRouter {
    /// Creates a new `WebRouter` with an empty route map.
    ///
    /// # Returns
    ///
    /// - `WebRouter` - A new instance of `WebRouter`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::router::WebRouter;
    ///
    /// let router = WebRouter::new();
    ///
    /// assert!(router.routes.is_empty());
    /// ```
    pub fn new() -> WebRouter {
        return WebRouter {
            routes: hashmap! {},
        };
    }

    /// Adds a new route to the `routes` hashmap using route path, method and route handler as input
    ///
    /// # Arguments
    ///
    /// - `path` - The route path as a `String`.
    /// - `method` - The HTTP method for the route as an `HttpMethod`.
    /// - `handler` - The `RouteHandler` for the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::{context::Context, response::Response, router::{WebRouter, RouteHandler}, utils::HttpMethod, utils::HttpStatusCode};
    ///
    /// let mut router = WebRouter::new();
    /// router.add("/".to_string(), HttpMethod::GET, RouteHandler::new(|ctx: Context| {
    ///     Response::new(HttpStatusCode::OK, "Hello, World!".to_string())
    /// }));
    /// ```
    pub fn add(&mut self, path: String, method: utils::HttpMethod, handler: RouteHandler) {
        self.routes
            .entry(path.to_string())
            .or_insert_with(HashMap::new)
            .insert(method.to_string(), handler);
    }

    /// Handles an incoming request and generates a response.
    ///
    /// This function handle response generation from request by first getting all the user-registered routes
    /// which match the request's path(it will be hashmap) from `routes` hashmap, then using that
    /// hashmap to get the route which matches request's method and then finaly using that route's
    /// handler function to generate the response for the request by providing a new `Context` with
    /// the request as input to the handler function
    ///
    /// # Arguments
    ///
    /// - `request` - The incoming `Request`.
    ///
    /// # Returns
    ///
    /// - `Response` - The generated response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::{context::Context, request::Request, response::Response, router::{WebRouter, RouteHandler}, utils::HttpMethod, utils::HttpStatusCode};
    ///
    /// let mut router = WebRouter::new();
    /// router.add("/".to_string(), HttpMethod::GET, RouteHandler::new(|ctx: Context| {
    ///     Response::new(HttpStatusCode::OK, "Hello, World!".to_string())
    /// }));
    ///
    /// let request = Request::new(&vec![
    ///     "GET / HTTP/1.1".to_string(),
    ///     "".to_string()
    /// ]).unwrap();
    ///
    /// let response = router.handle_request(request);
    ///
    /// assert_eq!(response.status_code, HttpStatusCode::OK);
    /// assert_eq!(response.body, "Hello, World!");
    /// ```
    pub fn handle_request(&self, request: request::Request) -> response::Response {
        match self.routes.get(&request.path) {
            Some(path_map) => match path_map.get(&request.method.to_string()) {
                Some(route_handler) => {
                    // the request path, method `exactly` matches a registered route path, method
                    return (route_handler.handler_func)(context::Context::new(request));
                }
                None => {
                    // the request path `exactly` matches a registered route path but the method is
                    // different
                    return response::Response::new(
                        utils::HttpStatusCode::MethodNotAllowed,
                        format!("{}", utils::HttpStatusCode::MethodNotAllowed.code().0).to_string(),
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
                                // process and validate query parameters from request path
                                let mut query_params = HashMap::new();
                                match request.path.split('?').nth(1) {
                                    Some(query) => {
                                        for part in query.split('&') {
                                            let mut key_value = part.split('=');
                                            let key = key_value.next().unwrap_or("");
                                            let value = key_value.next().unwrap_or("");
                                            if key.is_empty() {
                                                // If the key is empty, return a bad request response
                                                return response::Response::new(
                                                    utils::HttpStatusCode::BadRequest,
                                                    format!(
                                                        "{}",
                                                        utils::HttpStatusCode::BadRequest.code().0
                                                    )
                                                    .to_string(),
                                                );
                                            }
                                            query_params.insert(key.to_string(), value.to_string());
                                        }
                                    }
                                    None => {}
                                }

                                let mut context = context::Context::new(request);
                                context.params = params;
                                context.query_params = query_params;

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
                return response::Response::new(
                    utils::HttpStatusCode::NotFound,
                    format!("{}", utils::HttpStatusCode::NotFound.code().0).to_string(),
                );
            }
        }
    }
    /// Matches a request path to a registered dynamic route path, extracting parameters if available.
    ///
    /// This function first removes the query parameters from the request path string, then
    /// splits both the request path and route path into vectors by splitting at `/` (slashes).
    /// It ensures the lengths of these vectors are the same. If they are, it zips the vectors
    /// into one vector with the format `(request_path_part, route_path_part)`.
    ///
    /// It then loops over this vector and checks if the `route_path_part` of any item starts with `:`.
    /// If it does, this registered route is identified as a dynamic route, so the corresponding
    /// `request_path_part` is stored in the `params` `HashMap` which is then returned after the loop ends.
    /// If the `route_path_part` does not start with `:`, it is treated as a normal route and both parts
    /// must be equal. If they aren't, the function returns `None`.
    ///
    /// # Arguments
    ///
    /// - `request_path` - A `String` representing the path of the incoming request.
    /// - `route_path` - A `String` representing a registered route path pattern.
    ///
    /// # Returns
    ///
    /// An `Option<HashMap<String, String>>` containing the extracted parameters if the request path
    /// matches the registered route path pattern, or `None` if it does not match.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use browzer_web::router::WebRouter;
    ///
    /// let request_path = "/users/123".to_string();
    /// let route_path = "/users/:id".to_string();
    /// let params = WebRouter::match_dynamic_route(request_path, route_path).unwrap();
    ///
    /// assert_eq!(params.get("id"), Some(&"123".to_string()));
    /// ```
    fn match_dynamic_route(
        request_path: String,
        route_path: String,
    ) -> Option<HashMap<String, String>> {
        let mut params: HashMap<String, String> = hashmap! {};

        let request_path_parts: Vec<&str> = request_path.split('?').collect::<Vec<_>>()[0]
            .split('/')
            .collect();
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
