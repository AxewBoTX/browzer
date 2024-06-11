mod utils;

use browzer_web;

use crate::utils::base::*;

fn main() {
    let mut server = browzer_web::WebServer::new(format!("0.0.0.0:{}", PORT), 5);
    server.get("/", |mut c| {
        return c.send_string(browzer_web::utils::HttpStatusCode::OK, "Hello, World!");
    });
    server.get("/users/:username", |mut c| {
        println!("Dynamic Parameters: {:#?}", c.params);
        println!("Query Parameters: {:#?}", c.query_params);
        return c.send_string(browzer_web::utils::HttpStatusCode::OK, "Hello, World!");
    });
    server.listen();
}
