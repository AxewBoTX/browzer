mod utils;

use web;

use crate::utils::base::*;
use std::{thread, time::Duration};

fn main() {
    let mut server = web::WebServer::new(format!("0.0.0.0:{PORT}"), 5);
    server.get("/", |mut c| {
        c.send_string(web::response::HttpStatusCode::OK, "<h1>Hello, World!</h1>")
    });
    server.get("/users/data", |mut c| {
        c.send_string(web::response::HttpStatusCode::OK, "<h1>User: John Doe</h1>")
    });
    server.get("/sleep", |mut c| {
        thread::sleep(Duration::from_secs(5));
        c.send_string(web::response::HttpStatusCode::OK, "<h1>Sleeping...</h1>")
    });
    server.post("/fetch-data", |mut c| {
        c.send_string(
            web::response::HttpStatusCode::OK,
            "Response to POST request to /fetch-data",
        )
    });
    server.listen();
}
