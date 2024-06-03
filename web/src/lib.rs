pub mod context;
pub mod error;
pub mod request;
pub mod response;
pub mod router;
pub mod utils;

// internal crate imports
use crate::{context::*, error::*, request::*, response::*, router::*};

// standard library imports
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

// ----- WebServer struct
#[derive(Debug)]
pub struct WebServer {
    pub listener: TcpListener,
    request_pool: utils::thread_pool::ThreadPool,
    pub hide_banner: bool,
    pub address: String,
    router: Arc<WebRouter>,
}

impl WebServer {
    // create a `TcpListener`, bind it to the address provided, create a `ThreadPool` with user
    // defined number of workers which handles distribution of requests to worker threads and
    // return the `WebServer` object
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
            router: Arc::new(WebRouter::new()),
        };
    }

    // functions to register routes to the `routes` hashmap of the `WebRouter` using request methods, path, and handler functions
    // ----- GET request
    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> Response + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => router.add(
                path.to_string(),
                HttpMethod::GET,
                RouteHandler::new(handler),
            ),
            None => eprintln!(
                "{}",
                WebServerError::InternalServerError("WebRouter is not innitialized".to_string())
            ),
        };
    }
    // ----- POST request
    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> Response + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => router.add(
                path.to_string(),
                HttpMethod::POST,
                RouteHandler::new(handler),
            ),
            None => eprintln!(
                "{}",
                WebServerError::InternalServerError("WebRouter is not innitialized".to_string())
            ),
        };
    }
    // ----- PATCH request
    pub fn patch<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> Response + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => router.add(
                path.to_string(),
                HttpMethod::PATCH,
                RouteHandler::new(handler),
            ),
            None => eprintln!(
                "{}",
                WebServerError::InternalServerError("WebRouter is not innitialized".to_string())
            ),
        };
    }
    // ----- DELETE request
    pub fn delete<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> Response + 'static + Send + Sync,
    {
        match Arc::get_mut(&mut self.router) {
            Some(router) => router.add(
                path.to_string(),
                HttpMethod::DELETE,
                RouteHandler::new(handler),
            ),
            None => eprintln!(
                "{}",
                WebServerError::InternalServerError("WebRouter is not innitialized".to_string())
            ),
        };
    }

    // listen for incoming
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
    // handle various operations related to incoming requests
    fn handle_request(router: Arc<WebRouter>, mut stream: TcpStream) -> Result<(), WebServerError> {
        let buf_reader = BufReader::new(&mut stream);

        // parse the request string into a `Request` struct by first parsing the string to a string
        // vector containling the lines of requests as elements and then passing that vector onto the
        // `new` function of the `Request` string as input
        let request = match Request::new(&match buf_reader
            .lines()
            .take_while(|result| match result {
                Ok(line) => !line.is_empty(),
                Err(_) => false,
            })
            .collect()
        {
            Ok(request) => request,
            Err(e) => return Err(WebServerError::IO(e)),
        }) {
            Ok(safe) => safe,
            Err(e) => {
                return Err(WebServerError::RequestParseError(e));
            }
        };

        // utilize user registered routes from `routes` hashmap in the `WebRouter` to handle
        // requests, generate responses and then send those responses to the request agent throught
        // the TCP connection stream
        match stream.write_all(router.handle_request(request).to_string().as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(WebServerError::IO(e));
            }
        };

        match stream.flush() {
            Ok(_) => Ok({}),
            Err(e) => {
                return Err(WebServerError::StreamFlushError(e.to_string()));
            }
        }
    }
}
