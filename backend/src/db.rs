use mongodb::{Client, Database};
use std::env;

pub async fn get_db() -> Database {
    let client_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&client_uri).await.expect("Failed to initialize MongoDB client");
    client.database("chat_db")
}