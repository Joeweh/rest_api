use std::str::FromStr;

use futures::stream::TryStreamExt;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use mongodb::{bson::{doc, Document, oid::ObjectId}, Cursor, Collection, options::ClientOptions, Client, Database};
use serde::Deserialize;
use serde::Serialize;
use actix_cors::Cors;

#[derive(Serialize, Deserialize)]
struct User {
    username: String
}

#[get("/api/users")]
async fn get_users(db_client: web::Data<Client>) -> impl Responder {
    let database: Database = db_client.database("testDB");

    let collection: Collection<Document> = database.collection::<Document>("users");

    let cursor: Cursor<Document> = collection.find(None, None).await.unwrap();

    let list: Vec<Document> = cursor.try_collect().await.unwrap();

    HttpResponse::Ok().json(list)
}

#[get("/api/users/{uid}")]
async fn get_user_by_uid(db_client: web::Data<Client>, uid: web::Path<String>) -> impl Responder {
    let database: Database = db_client.database("testDB");

    let collection: Collection<Document> = database.collection::<Document>("users");

    let user: Document = collection.find_one(doc! { "_id": ObjectId::from_str(&uid).unwrap() }, None).await.unwrap().unwrap();

    HttpResponse::Ok().json(user)
}

#[post("/api/users")]
async fn save_user(db_client: web::Data<Client>, body: web::Json<User>) -> impl Responder {
    let database: Database = db_client.database("testDB");

    let collection: Collection<Document> = database.collection::<Document>("users");

    collection.insert_one(doc! { "username": body.username.as_str() }, None).await.unwrap();

    HttpResponse::Ok().json(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://admin:admin@cluster0.l4l1afu.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let client = Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        Cors::permissive();
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(save_user)
            .service(get_user_by_uid)
            .service(get_users)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}