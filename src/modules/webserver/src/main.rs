// Webserver main

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

// Internal import
#[path = "../../intro/src/main.rs"]
mod main;
use main::gcd;

// Post data struct
#[derive(Deserialize)]
struct GcdParams {
    n: u64,
    m: u64,
}

// Response indexing
fn get_index() -> HttpResponse {
    return HttpResponse::Ok().content_type("text/html").body(r#"
        <title> GCD Calculator </title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit"> Compute GCD </button>
        </form?
    "#);
}

// Post handler
fn post_gcd(form: web::Form<GcdParams>) -> HttpResponse {
    // Bad request response
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Please use non-zero values...")
    }

    // Generate valid request response
    let response = format!("The greatest common divisor of {} and {} / is <b>{}</b>", form.n, form.m, gcd(form.n, form.m));
    return HttpResponse::Ok().content_type("text/html").body(response);
} 

// App
pub fn main() {
    // Configure app server
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    // Launch app server
    println!("Serving on http://localhost:3000...");
    server.bind("127.0.0.1:3000").expect("Error binding to server address").run().expect("Error running server");
}
