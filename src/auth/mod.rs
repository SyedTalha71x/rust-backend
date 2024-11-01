use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::{bson::doc, Database};
use crate::models::{SignupRequest, LoginRequest, User, UserResponse};
use serde_json::json;

#[post("/signup")]
pub async fn signup(
    db: web::Data<Database>, 
    signup_data: web::Json<SignupRequest>
) -> impl Responder {
    let hashed_password = hash(&signup_data.password, DEFAULT_COST).unwrap();
    let new_user = User {
        id: None,
        username: signup_data.username.clone(),
        password: hashed_password,
    };

    let collection = db.collection::<User>("users");

    if collection.find_one(doc! { "username": &new_user.username }, None).await.unwrap().is_some() {
        return HttpResponse::BadRequest().json(json!({"error": "User already exists"}));
    }

    collection.insert_one(new_user, None).await.unwrap();
    
    HttpResponse::Created().json(json!({"message": "User created successfully"}))
}

#[post("/login")]
pub async fn login(
    db: web::Data<Database>, 
    login_data: web::Json<LoginRequest>
) -> impl Responder {
    let collection = db.collection::<User>("users");


    let user = collection.find_one(doc! { "username": &login_data.username }, None).await.unwrap();

    match user {
        Some(user) => {
            if verify(&login_data.password, &user.password).unwrap() {
                let response = UserResponse {
                    id: user.id.unwrap(),
                    username: user.username,
                };
                HttpResponse::Ok().json(response)
            } else {
                HttpResponse::Unauthorized().json(json!({"error": "Invalid password"}))
            }
        }
        None => HttpResponse::NotFound().json(json!({"error": "User not found"})),
    }
}
