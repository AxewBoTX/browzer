mod utils;

use web;

use crate::utils::base::*;
use std::{thread, time::Duration};

fn main() {
    let mut server = web::WebServer::new(format!("0.0.0.0:{PORT}"));
    server.get("/", |_| {
        web::response::Response::new(
            web::response::HttpStatusCode::OK,
            "<h1>Hello, World!</h1>".to_string(),
        )
    });
    server.get("/users/data", |_| {
        web::response::Response::new(
            web::response::HttpStatusCode::OK,
            "<h1>User: John Doe</h1>".to_string(),
        )
    });
    server.get("/sleep", |_| {
        thread::sleep(Duration::from_secs(5));
        web::response::Response::new(
            web::response::HttpStatusCode::OK,
            "<h1>Sleeping...</h1>".to_string(),
        )
    });
    server.post("/fetch-data", |_| {
        web::response::Response::new(
            web::response::HttpStatusCode::OK,
            "Response to POST request to '/fetch-data' route".to_string(),
        )
    });
    server.listen();
}
