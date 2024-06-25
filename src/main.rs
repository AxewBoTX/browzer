mod utils;

use browzer_web;

fn main() {
    let mut server = browzer_web::WebServer::new(format!("0.0.0.0:{}", utils::PORT), 5);

    server.get("/", |mut c| {
        return c.send_string(browzer_web::utils::HttpStatusCode::OK, "Hello,World!");
    });

    server.listen();
}
