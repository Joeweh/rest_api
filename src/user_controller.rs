use actix_web::{get, web, Responder, HttpResponse, post, put, delete};

use crate::user_repo::UserRepository;

use crate::user::{NewUser, User};

#[get("/api/users")]
async fn get_users(user_repo: web::Data<UserRepository>) -> impl Responder {
    let users: Vec<User> = user_repo.get_users().await;

    HttpResponse::Ok().json(users)
}

#[get("/api/users/{uid}")]
async fn get_user(user_repo: web::Data<UserRepository>, uid: web::Path<String>) -> impl Responder {
    let result: Option<User> = user_repo.get_user(uid.to_string()).await;

    if result.is_none() {
        HttpResponse::NotFound().body("No User With id: {&uid}");
    }

    HttpResponse::Ok().json(result.unwrap())
}

#[post("/api/users")]
async fn save_user(user_repo: web::Data<UserRepository>, new_user: web::Json<NewUser>) -> impl Responder {    
    user_repo.save_user(&new_user).await;

    HttpResponse::Created()
}

#[put("/api/users/{uid}")]
async fn update_user(user_repo: web::Data<UserRepository>, uid: web::Path<String>, new_user: web::Json<NewUser>) -> impl Responder {
    if user_repo.get_user(uid.to_string()).await.is_none() {
        HttpResponse::NotFound().body("No User With id: {&uid}");
    }

    user_repo.update_user(uid.to_string(), &new_user).await;

    HttpResponse::NoContent()
}

#[delete("/api/users/{uid}")]
async fn delete_user(user_repo: web::Data<UserRepository>, uid: web::Path<String>) -> impl Responder {
    if user_repo.get_user(uid.to_string()).await.is_none() {
        HttpResponse::NotFound().body("No User With id: {&uid}");
    }

    user_repo.delete_user(uid.to_string()).await;
    
    HttpResponse::NoContent()
}