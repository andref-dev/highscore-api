use actix_web::{Responder, HttpResponse, App, web, HttpServer};


async fn health_handler() -> impl Responder {
    println!("Health handler executed sucessfully");
    HttpResponse::Ok().body("{\"status\": \"pass\"}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on http://localhost:4000");
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_handler))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
