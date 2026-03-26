use mongodb::{Client,Database};
use std::env;

async fn connect_db() -> Database {
    let uri = env::var("MONGO_URI").except("the Mongo Url is not working")
    let client = Client::with_uri_str(&uri).await.except("Failed to connect to db")
    Client.database(Ecom)
}