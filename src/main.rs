use actix_web::{web, App, HttpResponse, HttpServer, Responder};

//async fn health_check() -> impl Responder {
async fn health_check() -> HttpResponse {
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

#[cfg(test)]
mod tests {
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succeds() {
        let response = health_check().await;

        // this requires chaging the response type to
        // HttpResponse to compile
        // Need to importit with `use actix_web::HttpResponse`
        assert!(response.status().is_success())
    }
}
