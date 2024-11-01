use mongodb::{Client, Database};
use std::env;

pub async fn connect_to_db() -> Database {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri).await.expect("Failed to initialize MongoDB client");
    client.database("RustAuth") 
}
