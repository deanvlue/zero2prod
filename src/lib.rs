// src/lib.rs
use actix_web::{web, App, HttpResponse, HttpServer};

// adding Server to run it without awaiting it
use actix_web::dev::Server;

//async fn health_check() -> impl Responder {
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// The signature of the app changes
// We return `Server` on the happy path and we dropped the async keyword
// We have no await call so we ditch it
pub fn run(address: &str) -> Result<Server, std::io::Error> {
    println!("Server running at http://0.0.0.0:9090");
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(address)?
        .run();

    // NO wait here
    Ok(server)
}
