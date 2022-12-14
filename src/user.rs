use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String
}