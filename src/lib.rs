// src/lib.rs
use actix_web::{web, App, HttpResponse, HttpServer};

// adding Server to run it without awaiting it
use actix_web::dev::Server;

//async fn health_check() -> impl Responder {
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We need to mark run as public.
// It is no longer a binary entrpoint, we can mark it as async
// withouth having to  use any proc-macro incantation
pub async fn run() -> Result<(), std::io::Error> {
    println!("Server running at http://0.0.0.0:9090");
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
