mod utils;

use web;

use crate::utils::base::*;

fn main() {
    let mut server = web::WebServer::new(format!("0.0.0.0:{PORT}"), 5);
    server.get("/", |mut c| {
        c.send_string(web::response::HttpStatusCode::OK, "<h1>Hello, World!</h1>")
    });
    server.get("/users/", |mut c| {
        c.send_string(web::response::HttpStatusCode::OK, "<h1>User: John Doe</h1>")
    });
    server.get("/users/:UserID", |mut c| {
        println!("{:#?}", c.params);
        c.send_string(
            web::response::HttpStatusCode::OK,
            "<h1>Dynamic Route: Users</h1>",
        )
    });
    server.get("/users/:UserID/posts/:PostID", |mut c| {
        println!("{:#?}", c.params);
        c.send_string(
            web::response::HttpStatusCode::OK,
            "<h1>Nested Dynamic Route: Users,Posts</h1>",
        )
    });
    server.post("/fetch-data", |mut c| {
        c.send_string(
            web::response::HttpStatusCode::OK,
            "Response to POST request to /fetch-data",
        )
    });
    server.listen();
}
