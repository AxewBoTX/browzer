pub mod error;
pub mod request;
pub mod response;
pub mod utils;

// internal crate imports
use crate::{error::*, request::*, response::*};

// standard library imports
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// ----- WebServer struct
#[derive(Debug)]
pub struct WebServer {
    pub listener: TcpListener,
    request_pool: utils::thread_pool::ThreadPool,
    pub hide_banner: bool,
    pub address: String,
}

impl WebServer {
    // create a `TcpListener`, bind it to the address provided, create a `ThreadPool` which handles
    // distributing requests to worker threads, and return the `WebServer` object
    pub fn new(address: String) -> WebServer {
        let listener = match TcpListener::bind(&address) {
            Ok(listener) => listener,
            Err(listener_create_err) => {
                panic!(
                    "Failed to create listener for the WebServer, Error: {}",
                    listener_create_err.to_string()
                );
            }
        };

        let request_pool = utils::thread_pool::ThreadPool::new(5);

        // return the WebServer struct
        return WebServer {
            listener,
            request_pool,
            hide_banner: false,
            address,
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
            match stream {
                Ok(stream) => {
                    match self.request_pool.execute(|| {
                        match handle_request(stream) {
                            Ok(_) => {}
                            Err(e) => {
                                eprint!("Failed to handle incoming request, Error: {e}");
                            }
                        };
                    }) {
                        Ok(_) => {}
                        Err(e) => eprint!(
                            "Failed to assign Worker thread to incoming request, Error: {}",
                            e.to_string()
                        ),
                    };
                }
                Err(e) => {
                    eprint!("Failed to establish a connection, Error: {}", e.to_string());
                }
            }
        }
    }
}

// handle various operations related to incoming requests
fn handle_request(mut stream: TcpStream) -> Result<(), WebServerError> {
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
    // log the Request struct
    println!("{:#?}", request);

    // dummy response generation using `Respose` struct`
    let response = match request.method {
        HttpMethod::GET => {
            if request.path == "/" {
                Response::new(HttpStatusCode::OK, "<h1>Hello, World!</h1>".to_string())
            } else if request.path == "/sleep" {
                thread::sleep(Duration::from_secs(5));
                Response::new(HttpStatusCode::OK, "<h1>Sleeping...</h1>".to_string())
            } else {
                Response::new(HttpStatusCode::NotFound, "<h1>Not Found!</h1>".to_string())
            }
        }
        _ => Response::new(
            HttpStatusCode::NotImplemented,
            "<h1>Not Implemented!</h1>".to_string(),
        ),
    };
    match stream.write_all(response.to_string().as_bytes()) {
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
