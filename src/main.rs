mod user;
mod user_repo;
mod user_controller;

use actix_web::{web, App, HttpServer, get, Responder, HttpResponse};
use mongodb::{options::ClientOptions, Client};

use actix_cors::Cors;
use user_controller::{save_user, update_user, delete_user, get_user, get_users};
use user_repo::UserRepository;

use std::env;
use std::time::Duration;
use actix_rt::{spawn, time};

#[get("/api")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://admin:admin@cluster0.xx20tsi.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

    let client: Client = Client::with_options(client_options).unwrap();

    let port: u16 = match env::var("PORT") {
        Ok(port_string) => port_string.parse::<u16>().unwrap(),
        Err(_error) => 8080
    };

    // Ping server periodically to prevent cold starts
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5 * 60));
        loop {
            interval.tick().await;
            reqwest::get("https://rest-api-sm99.onrender.com/api").await.unwrap();
        }
    });

    HttpServer::new(move || {
        Cors::permissive();
        App::new()
            .app_data(web::Data::new(UserRepository::new(client.clone())))
            
            .service(save_user)
            .service(update_user)
            .service(delete_user)
            .service(get_user)
            .service(get_users)

            .service(health_check)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}