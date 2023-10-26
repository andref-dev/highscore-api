use actix_web::{Responder, HttpResponse, App, web, HttpServer};


async fn health_handler() -> impl Responder {
    println!("Health handler executed successfully");
    HttpResponse::Ok().body("{\"status\": \"pass\"}")
}

async fn echo_handler(req_body: String) -> impl Responder {
    println!("Echo handler executed successfully");
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on http://localhost:4000");
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_handler))
            .route("/echo", web::post().to(echo_handler))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
