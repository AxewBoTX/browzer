mod utils;

use browzer_web;

use crate::utils::base::*;

fn main() {
    let mut server = browzer_web::WebServer::new(format!("0.0.0.0:{}", PORT), 5);

    // server.middleware(|c| {
    //     println!("{:#?}", c.request);
    //     return c;
    // });

    server.get("/", |mut c| {
        return c.send_string(
            browzer_web::utils::HttpStatusCode::OK,
            r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Form</title>
            <script>
                function submitForm(event) {
                    event.preventDefault();
                    const formData = new FormData(event.target);
                    fetch('/', {
                        method: 'POST',
                        body: new URLSearchParams(formData)
                    })
                    .then(response => response.text())
                    .then(data => {
                        console.log('Success:', data);
                    })
                    .catch((error) => {
                        console.error('Error:', error);
                    });
                }
            </script>
        </head>
        <body>
            <h1>Form</h1>
            <form onsubmit="submitForm(event)">
                <input type="text" id="username" name="username" required>
                <input type="text" id="email" name="email" required>
                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
    "#,
        );
    });
    server.post("/", |mut c| {
        println!("Username: {}", c.form_value("username"));
        return c.send_string(
            browzer_web::utils::HttpStatusCode::OK,
            "Successful Post Request",
        );
    });
    server.patch("/", |mut c| {
        return c.send_string(
            browzer_web::utils::HttpStatusCode::OK,
            "Successful Patch Request",
        );
    });
    server.delete("/", |mut c| {
        return c.send_string(
            browzer_web::utils::HttpStatusCode::OK,
            "Successful Delete Request",
        );
    });
    server.listen();
}
