use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("API Online!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}