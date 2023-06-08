// Webserver main
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[path = "endpoints.rs"]
mod endpoints;
use endpoints::gcd;

// Post data struct
#[derive(Deserialize)]
pub struct GcdParams {
    n: u64,
    m: u64,
}

// Response indexing
pub fn get_index() -> HttpResponse {
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
pub fn post_gcd(form: web::Form<GcdParams>) -> HttpResponse {
    // Bad request response
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Please use non-zero values...")
    }

    // Generate valid request response
    let response = format!("The greatest common divisor of {} and {} / is <b>{}</b>", form.n, form.m, gcd(form.n, form.m));
    return HttpResponse::Ok().content_type("text/html").body(response);
} 

