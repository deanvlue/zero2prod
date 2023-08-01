// src/lib.rs
use actix_web::{web, App, HttpResponse, HttpServer};

// adding Server to run it without awaiting it
use actix_web::dev::Server;

// Usign TCPListener to check the port this is running
use std::net::TcpListener;

//async fn health_check() -> impl Responder {
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    println!("Server at http://0.0.0.0:{}", port);
    // NO wait here
    Ok(server)
}
