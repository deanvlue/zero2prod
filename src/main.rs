use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello, {}", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Server running at http://0.0.0.0:9090");
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
