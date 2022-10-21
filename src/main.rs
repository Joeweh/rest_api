use std::str::FromStr;

use futures::stream::{TryStreamExt};

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use mongodb::{bson::{doc, Document, oid::ObjectId}, Cursor, Collection, options::ClientOptions, Client, Database};

#[get("/api/users")]
async fn get_users() -> impl Responder {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://admin:admin@cluster0.l4l1afu.mongodb.net/?retryWrites=true&w=majority").await.unwrap();
    
    let client: Client = Client::with_options(client_options).unwrap();

    let database: Database = client.database("testDB");

    let collection: Collection<Document> = database.collection::<Document>("users");

    let cursor: Cursor<Document> = collection.find(None, None).await.unwrap();

    let list: Vec<Document> = cursor.try_collect().await.unwrap();

    HttpResponse::Ok().json(list)
}

#[get("/api/users/{uid}")]
async fn get_user_by_uid(uid: web::Path<String>) -> impl Responder {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://admin:admin@cluster0.l4l1afu.mongodb.net/?retryWrites=true&w=majority").await.unwrap();
    
    let client: Client = Client::with_options(client_options).unwrap();

    let database: Database = client.database("testDB");

    let collection: Collection<Document> = database.collection::<Document>("users");

    let user: Document = collection.find_one(doc! { "_id": ObjectId::from_str(&uid).unwrap() }, None).await.unwrap().unwrap();

    HttpResponse::Ok().json(user)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user_by_uid)
            .service(get_users)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}