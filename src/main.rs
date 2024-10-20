use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

// curl -v http://127.0.0.1:8000/health_check