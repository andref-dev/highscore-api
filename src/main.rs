use actix_web::{Responder, HttpResponse, App, web, HttpServer};


async fn health_handler() -> impl Responder {
    HttpResponse::Ok().body("{\"status\": \"pass\"}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_handler))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
