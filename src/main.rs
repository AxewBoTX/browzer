mod utils;

use std::{
    io::{BufRead, BufReader, Write},
    net::*,
    thread,
    time::Duration,
};

use web;

use crate::utils::base::*;

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{PORT}")).unwrap();
    println!("-----> HTTP server running on 0.0.0.0:{PORT}");

    let request_pool = web::utils::thread_pool::ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        request_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // get data
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("HTTP Request: {:#?}", http_request);
    let request_line = http_request
        .join("\r\n")
        .lines()
        .next()
        .unwrap()
        .trim()
        .to_string();

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
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
