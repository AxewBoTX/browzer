pub mod error;
pub mod utils;

// standard library imports
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// internal crate imports
use crate::error::*;

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

        WebServer {
            listener,
            request_pool,
            hide_banner: false,
            address,
        }
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

    // convert the text request into a vector
    let http_request: Vec<_> = match buf_reader
        .lines()
        .take_while(|result| match result {
            Ok(line) => !line.is_empty(),
            Err(_) => false,
        })
        .collect()
    {
        Ok(request) => request,
        Err(e) => return Err(WebServerError::IO(e)),
    };
    println!("HTTP Request: {:#?}", http_request);

    let request_line = match http_request.join("\r\n").lines().next() {
        Some(line) => line.trim().to_string(),
        None => return Err(WebServerError::EmptyRequestError),
    };

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "<h1>Hello, World!<h1>"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "<h1>Sleeping...<h1>")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "<h1>Not Found!<h1>"),
    };
    let content_length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
    stream
        .write_all(response.as_bytes())
        .map_err(|e| WebServerError::IO(e))?;
    stream
        .flush()
        .map_err(|e| WebServerError::StreamFlushError(e.to_string()))
}
