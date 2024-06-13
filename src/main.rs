mod utils;

use browzer_web;

use crate::utils::base::*;

fn main() {
    let mut server = browzer_web::WebServer::new(format!("0.0.0.0:{}", PORT), 5);
    server.get("/", |mut c| {
        return c.send_string(browzer_web::utils::HttpStatusCode::OK, "(/) route");
    });
    server.get("/users", |mut c| {
        return c.send_string(browzer_web::utils::HttpStatusCode::OK, "/users route");
    });
    server.listen();
}
