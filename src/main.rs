mod utils;

use web;

use crate::utils::base::*;

fn main() {
    let server = web::WebServer::new(format!("0.0.0.0:{PORT}"));
    server.listen();
}
