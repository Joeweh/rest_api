use std::str::FromStr;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use mongodb::{bson::{doc, Document, oid::ObjectId}, options::ClientOptions, Client};

#[get("/")]
async fn root() -> impl Responder {
    let client_options = ClientOptions::parse("mongodb+srv://admin:admin@cluster0.l4l1afu.mongodb.net/?retryWrites=true&w=majority").await;
    
    let client = Client::with_options(client_options.unwrap()).unwrap();

    let database = client.database("testDB");

    let collection = database.collection::<Document>("users");

    let user = collection.find_one(doc! { "_id": ObjectId::from_str("634f9e184b67156408039b7e").unwrap() }, None).await.unwrap().unwrap_or_else(|| doc! { "username": "error"});

    HttpResponse::Ok().json(user)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}