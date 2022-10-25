use std::str::FromStr;

use futures::TryStreamExt;
use mongodb::Client;
use mongodb::Collection;
use mongodb::Cursor;
use mongodb::Database;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::DeleteResult;
use mongodb::results::InsertOneResult;
use mongodb::results::UpdateResult;

use crate::user::NewUser;
use crate::user::User;

pub struct UserRepository {
    pub client: Client
}

impl UserRepository {
    pub fn new(client: Client) -> Self {
        UserRepository { client }
    }
    
    pub async fn get_users(&self) -> Vec<User> {
        let database: Database = self.client.database("testDB");
 
        let collection: Collection<User> = database.collection::<User>("users");

        let cursor: Cursor<User> = collection.find(None, None).await.unwrap();

        return cursor.try_collect().await.unwrap();
    }

    pub async fn get_user(&self, uid: String) -> Option<User> {
        let database: Database = self.client.database("testDB");

        let collection: Collection<User> = database.collection::<User>("users");

        let result: Result<Option<User>, mongodb::error::Error> = collection.find_one(doc! { "_id": ObjectId::from_str(&uid).unwrap() }, None).await;

        let user: Option<User> = match result {
            Ok(option) => option,
            Err(_err) => None,
        };

        return user;
    }

    pub async fn save_user(&self, new_user: &NewUser) -> InsertOneResult {
        let database: Database = self.client.database("testDB");

        let collection: Collection<User> = database.collection::<User>("users");

        let user = User {
            _id: ObjectId::new(), 
            username: new_user.username.to_string()
        };

        return collection.insert_one(user, None).await.unwrap();
    }

    pub async fn update_user(&self, uid: String, new_user: &NewUser) -> UpdateResult {
        let database: Database = self.client.database("testDB");

        let collection: Collection<User> = database.collection::<User>("users");

        let user = User {
            _id: ObjectId::from_str(&uid).unwrap(),
            username: new_user.username.to_string()
        };

        return collection.replace_one(doc! { "_id": user._id }, user, None).await.unwrap();
    }

    pub async fn delete_user(&self, uid: String) -> DeleteResult {
        let database: Database = self.client.database("testDB");

        let collection: Collection<User> = database.collection::<User>("users");

        return collection.delete_one(doc! { "_id": ObjectId::from_str(&uid).unwrap() }, None).await.unwrap();
    }
}