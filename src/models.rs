use serde::{Deserialize, Serialize};
use mongodb::bson::{self, oid::ObjectId};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: ObjectId,
    pub username: String,
}
