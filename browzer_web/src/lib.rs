//! # browzer_web
//!
//! `browzer_web` is a very simple framework for building web applications and backends.
//!
//! ## Examples
//!
//! ```rust
//! use browzer_web;
//!
//! fn main() {
//!     let mut server = browzer_web::WebServer::new(format!("0.0.0.0:{}", PORT), 5);
//!     server.get("/", |mut c| {
//!         return c.send_string(browzer_web::utils::HttpStatusCode::OK, "Hello, World!");
//!     });
//!     server.listen();
//! }
//! ```
//!
//! ## Modules
//!
//! - `context` - route context which helps to easily work with router handlers
//! - `error` - custom errors
//! - `request` - handle HTTP requests related functionality
//! - `response` - handle HTTP response related functionality
//! - `router` - deals with routing and other aspects of routing like middlewares, registered routes
//! - `utils` - utilities used by the framework

pub mod context;
pub mod error;
pub mod request;
pub mod response;
pub mod router;
pub mod utils;

// standard library imports
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::Arc,
};

/// Represents a web server.
///
/// The `WebServer` struct is responsible for creating the main server which binds all the
/// functionality of the web framework like routing, response generation, listening to
/// requests, etc together efficiently and properly.
///
/// # Fields
///
/// - `listener` - A `TcpListener` that listens for incoming requests streams.
/// - `request_pool`- A custom `ThreadPool` implementation which handles request distribution to various worker threads
/// - `hide_banner` - A boolean flag to control whether the server banner should be displayed(logged to the console) or not
/// - `address` - The address to which the WebServer binds the TcpListener
/// - `router` - An `Arc` wrapped `WebRouter` which is responsible for routing logic of the server
///
/// # Examples
///
/// ```rust
/// use browzer_web::WebServer;
///
/// let server = WebServer::new("127.0.0.1:8080".to_string(), 4);
/// server.listen();
/// ```
// ----- WebServer struct
#[derive(Debug)]
pub struct WebServer {
    pub listener: TcpListener,
    request_pool: utils::thread_pool::ThreadPool,
    pub hide_banner: bool,
    pub address: String,
    router: Arc<router::WebRouter>,
}

impl WebServer {
    /// Creates a new `WebServer` instance.
    ///
    /// Create a `TcpListener`, bind it to the address provided, create a `ThreadPool` with
    /// user-defined number of workers which handles distribution of requests to worker threads and
    /// return the `WebServer` object.
    ///
    /// # Arguments
    ///
    /// - `address` - A `String` representing the address on which the server will listen for
    /// incoming requests.
    /// - `workers` - A `usize` specifying the  number of worker threads that will be created in
    /// the thread pool, to which the incoming requets will be distributed.
    ///
    /// # Returns
    ///
    /// - `WebServer` - A new instance of `WebServer`.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to bind the `TcpListener` to the provided address.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use browzer_web::WebServer;
    ///
    /// let server = WebServer::new("127.0.0.1:8080".to_string(), 4);
    /// server.listen();
    /// ```
    pub fn new(address: String, workers: usize) -> WebServer {
        let listener = match TcpListener::bind(&address) {
            Ok(listener) => listener,
            Err(listener_create_err) => {
                panic!(
                    "Failed to create listener for the WebServer, Error: {}",
                    listener_create_err.to_string()
                );
            }
        };

        let request_pool = utils::thread_pool::ThreadPool::new(workers);

        // return the WebServer struct
        return WebServer {
            listener,
            request_pool,
            hide_banner: false,
            address,
            router: Arc::new(router::WebRouter::new()),
        };
    }

    /// Register a new middleware
    ///
    /// This method allows you to register a new middleware function in the ruoter's middleware
    /// vector, which applies all your registered middlewares to incoming requests one-by-one in
    /// exact order in which you defined those middleware functions
    ///
    /// # Arguments
    ///
    /// - `middleware_func` - A closure function containing the functionality of the middleware
    /// defined by the user
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server = WebServer::new("127.0.0.1:8080".to_string(), 4);
    ///
    /// server.middleware(|mut ctx| {
    ///     // some functionality
    ///     return ctx
    /// });
    /// ```
    ///
    /// # Errors
    ///
    /// If the router is not initialized, this method will print an error message using `eprintln!`.
    ///
    /// # Panics
    ///
    /// This function will not panic under normal conditions. However, if the router is not properly
    /// initialized, it will log an error.
    pub fn middleware<F>(&mut self, middleware_func: F)
    where
        F: Fn(context::Context) -> context::Context + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => router.add_middleware(Box::new(middleware_func)),
            None => eprintln!(
                "{}",
                error::WebServerError::InternalServerError(
                    "WebRouter is not innitialized".to_string()
                )
            ),
        };
    }

    /// Registers a new route for handling HTTP GET requests.
    ///
    /// This method allows you to define a route and associate it with a handler function that
    /// will be called when a GET request is made to the specified path. The handler function
    /// should accept a `Context` object and return a `Response` object.
    ///
    /// # Arguments
    ///
    /// - `path` - A string slice that holds the path for the route. This is the URL path that will be
    ///   matched against incoming GET requests.
    /// - `handler` - A closure or function that takes a `Context` as input and returns a `Response`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server = WebServer::new("127.0.0.1:8080".to_string(), 4);
    ///
    /// server.get("/hello", |mut ctx| {
    ///     return ctx.send_string(browzer_web::utils::HttpStatusCode::OK, "Hello, World!");
    /// });
    /// ```
    ///
    /// # Errors
    ///
    /// If the router is not initialized, this method will print an error message using `eprintln!`.
    ///
    /// # Panics
    ///
    /// This function will not panic under normal conditions. However, if the router is not properly
    /// initialized, it will log an error.
    // ----- GET request
    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(context::Context) -> response::Response + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => {
                match router.add(path.to_string(), utils::HttpMethod::GET, Box::new(handler)) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                    }
                }
            }
            None => eprintln!(
                "{}",
                error::WebServerError::InternalServerError(
                    "WebRouter is not innitialized".to_string()
                )
            ),
        };
    }
    /// Registers a new route for handling HTTP POST requests.
    ///
    /// This method allows you to define a route and associate it with a handler function that
    /// will be called when a POST request is made to the specified path. The handler function
    /// should accept a `Context` object and return a `Response` object.
    ///
    /// # Arguments
    ///
    /// - `path` - A string slice that holds the path for the route. This is the URL path that will be
    ///   matched against incoming POST requests.
    /// - `handler` - A closure or function that takes a `Context` as input and returns a `Response`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server = WebServer::new("127.0.0.1:8080".to_string(), 4);
    ///
    /// server.post("/submit", |mut ctx| {
    ///     return ctx.send_string(browzer_web::utils::HttpStatusCode::OK, "Resource submitted!");
    /// });
    /// ```
    ///
    /// # Errors
    ///
    /// If the router is not initialized or it it fails to register the route using `WebRouter`,
    /// this method will print an error message using `eprintln!`.
    ///
    /// # Panics
    ///
    /// This function will not panic under normal conditions. However, if the router is not properly
    /// initialized, it will log an error.
    // ----- POST request
    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(context::Context) -> response::Response + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => {
                match router.add(path.to_string(), utils::HttpMethod::POST, Box::new(handler)) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                    }
                }
            }
            None => eprintln!(
                "{}",
                error::WebServerError::InternalServerError(
                    "WebRouter is not innitialized".to_string()
                )
            ),
        };
    }
    /// Registers a new route for handling HTTP PATCH requests.
    ///
    /// This method allows you to define a route and associate it with a handler function that
    /// will be called when a PATCH request is made to the specified path. The handler function
    /// should accept a `Context` object and return a `Response` object.
    ///
    /// # Arguments
    ///
    /// - `path` - A string slice that holds the path for the route. This is the URL path that will be
    ///   matched against incoming PATCH requests.
    /// - `handler` - A closure or function that takes a `Context` as input and returns a `Response`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server = WebServer::new("127.0.0.1:8080".to_string(), 4);
    ///
    /// server.patch("/update", |mut ctx| {
    ///     return ctx.send_string(browzer_web::utils::HttpStatusCode::OK, "Resource patched!");
    /// });
    /// ```
    ///
    /// # Errors
    ///
    /// If the router is not initialized or it it fails to register the route using `WebRouter`,
    /// this method will print an error message using `eprintln!`.
    ///
    /// # Panics
    ///
    /// This function will not panic under normal conditions. However, if the router is not properly
    /// initialized, it will log an error.
    // ----- PATCH request
    pub fn patch<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(context::Context) -> response::Response + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => {
                match router.add(
                    path.to_string(),
                    utils::HttpMethod::PATCH,
                    Box::new(handler),
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                    }
                }
            }
            None => eprintln!(
                "{}",
                error::WebServerError::InternalServerError(
                    "WebRouter is not innitialized".to_string()
                )
            ),
        };
    }
    /// Registers a new route for handling HTTP DELETE requests.
    ///
    /// This method allows you to define a route and associate it with a handler function that
    /// will be called when a DELETE request is made to the specified path. The handler function
    /// should accept a `Context` object and return a `Response` object.
    ///
    /// # Arguments
    ///
    /// - `path` - A string slice that holds the path for the route. This is the URL path that will be
    ///   matched against incoming DELETE requests.
    /// - `handler` - A closure or function that takes a `Context` as input and returns a `Response`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server = WebServer::new("127.0.0.1:8080".to_string(), 4);
    ///
    /// server.delete("/remove", |mut ctx|{
    ///     return ctx.send_string(browzer_web::utils::HttpStatusCode::OK, "Resource deleted!");
    /// });
    /// ```
    ///
    /// # Errors
    ///
    /// If the router is not initialized or it it fails to register the route using `WebRouter`,
    /// this method will print an error message using `eprintln!`.
    ///
    /// # Panics
    ///
    /// This function will not panic under normal conditions. However, if the router is not properly
    /// initialized, it will log an error.
    // ----- DELETE request
    pub fn delete<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(context::Context) -> response::Response + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => {
                match router.add(
                    path.to_string(),
                    utils::HttpMethod::DELETE,
                    Box::new(handler),
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                    }
                }
            }
            None => eprintln!(
                "{}",
                error::WebServerError::InternalServerError(
                    "WebRouter is not innitialized".to_string()
                )
            ),
        };
    }

    /// This method serves and maps static files from directory path to a route path
    ///
    /// This method does it's function by registering a dynamic GET method route to the
    /// `route_path`, that route's handler function gets the filename of the file that is requested
    /// from the dynamic route params and then check if a file with that name exists under the
    /// `dir_path`, if it does then the handler will return a `String` response with that file's
    /// content as body, it not then it returns a `NotFound`
    ///
    /// # Arguments
    ///
    /// - `dir_path` - A string representing the directory on the machine which the user wants to
    /// by served on the web app.
    /// - `route_path` - A string representing the path to which the user wants to map the
    /// static file directory
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server = WebServer::new("127.0.0.1:8080".to_string(), 4);
    ///
    /// server.serve_static("static","/static/get")
    /// ```
    pub fn serve_static(&mut self, dir_path: &str, route_path: &str) {
        let dir_path = Arc::new(dir_path.to_string());
        let dir_path_clone = Arc::clone(&dir_path);
        let route = format!("{}/:filename", route_path);

        self.get(&route, move |mut c| {
            let filename = match c.params.get("filename") {
                Some(filename) => filename,
                None => {
                    // Couldn't get the filename param
                    return c.send_string(
                        utils::HttpStatusCode::InternalServerError,
                        utils::HttpStatusCode::InternalServerError.code().0,
                    );
                }
            };
            let path = Path::new(&*dir_path_clone).join(filename); // NOTE: I have NO idea what is happening here
            match path.exists() {
                true => {
                    return c.send_string(
                        utils::HttpStatusCode::OK,
                        &match fs::read_to_string(path) {
                            Ok(res) => res,
                            Err(_) => {
                                // Couldn't prase the path to string
                                return c.send_string(
                                    utils::HttpStatusCode::InternalServerError,
                                    utils::HttpStatusCode::InternalServerError.code().0,
                                );
                            }
                        },
                    );
                }
                false => {
                    // filename doesn't exist under the dir_path
                    return c.send_string(
                        utils::HttpStatusCode::NotFound,
                        utils::HttpStatusCode::NotFound.code().0,
                    );
                }
            }
        });
    }

    /// Listens for incoming TCP connections and execute various functionality on those connections.
    ///
    /// This method starts the web server, accepting incoming connections and distributing
    /// them to worker threads for handling. It uses the `request_pool` to manage a pool of
    /// worker threads and assigns incoming requests to these workers. The function will
    /// continue to listen for connections indefinitely.
    ///
    /// # Panics
    ///
    /// This function will not panic under normal conditions. However, it will print error
    /// messages to the standard error output if it encounters issues with establishing connections
    /// or assigning worker threads.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server = WebServer::new("127.0.0.1:8080".to_string(), 4);
    /// server.listen();
    /// ```
    ///
    pub fn listen(&self) {
        // print the server banner( a simple log message ) accoding to the `address` field boolean variable
        if !self.hide_banner {
            println!("-----> HTTP server running on {}", self.address);
        }

        // loop over incoming requests and send those request as jobs to the `request_pool` in
        // order to be distributed to the worker threads
        for stream in self.listener.incoming() {
            let router = Arc::clone(&self.router);
            match stream {
                Ok(stream) => {
                    match self.request_pool.execute(|| {
                        match Self::handle_request(router, stream) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Failed to handle incoming request, Error: {}", e);
                            }
                        };
                    }) {
                        Ok(_) => {}
                        Err(e) => eprintln!(
                            "Failed to assign Worker thread to incoming request, Error: {}",
                            e.to_string()
                        ),
                    };
                }
                Err(e) => {
                    eprintln!("Failed to establish a connection, Error: {}", e.to_string());
                }
            }
        }
    }

    // handles various operations related to incoming requests.
    fn handle_request(
        router: Arc<router::WebRouter>,
        mut stream: TcpStream,
    ) -> Result<(), error::WebServerError> {
        let mut buf_reader = BufReader::new(&mut stream);

        // parse the request string into a `Request` struct by first parsing the string to a string
        // vector containling the lines of requests as elements by following cases:-
        //
        // - if the headers contain the `Content-Length` header and it's value is more than 0, then
        //   we properly parse the body too
        // - if the headers do not contain the `Content-Length` then we stop after parsing
        //
        // and then passing that vector onto the `new` function of the `Request` string as input
        let request = match request::Request::new(&{
            let mut request_vector = Vec::new();
            let mut content_length = 0;

            for line in buf_reader.by_ref().lines() {
                let line = match line {
                    Ok(ln) => ln,
                    Err(e) => return Err(error::WebServerError::IO(e)),
                };
                match line.strip_prefix("Content-Length: ") {
                    Some(c_l) => {
                        content_length = match c_l.trim().parse() {
                            Ok(safe_c_l) => safe_c_l,
                            Err(e) => return Err(error::WebServerError::from(e)),
                        }
                    }
                    None => {}
                }
                if line.is_empty() {
                    request_vector.push(line);
                    break;
                }
                request_vector.push(line);
            }
            let mut body = Vec::new();
            if content_length > 0 {
                body.resize(content_length, 0);
                match buf_reader.take(content_length as u64).read_exact(&mut body) {
                    Ok(_) => {}
                    Err(e) => return Err(error::WebServerError::IO(e)),
                }
                request_vector.push(String::from_utf8_lossy(&body).to_string());
            }
            request_vector // return the request_vector to Request::new() function
        }) {
            Ok(safe) => safe,
            Err(e) => {
                return Err(error::WebServerError::RequestParseError(e));
            }
        };

        // utilize user registered routes from `routes` hashmap in the `WebRouter` to handle
        // requests, generate responses and then send those responses to the request agent throught
        // the TCP connection stream
        match stream.write_all(
            match router.handle_request(request) {
                Ok(res) => res.to_string(),
                Err(e) => {
                    return Err(error::WebServerError::InternalServerError(e.to_string()));
                }
            }
            .as_bytes(),
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err(error::WebServerError::IO(e));
            }
        };

        match stream.flush() {
            Ok(_) => Ok({}),
            Err(e) => {
                return Err(error::WebServerError::StreamFlushError(e.to_string()));
            }
        }
    }
}
