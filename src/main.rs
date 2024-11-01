use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::Database;

mod models;
mod db;
mod auth; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = db::connect_to_db().await; 

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) 
            .service(auth::signup) 
            .service(auth::login)  
    })
    .bind("192.168.18.194:8080")? 
    .run()
    .await
}
