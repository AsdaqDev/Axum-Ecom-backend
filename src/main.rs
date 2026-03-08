use axum::Router;
use std::{net::SocketAddr, sync::Arc};
use mongodb::{Client, Database};

mod modules;

use modules::auth::{
    controller::routes,
    repository::authrepo,
    service::authservice,
};

#[tokio::main]
async fn main() {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("MongoDB connection failed");

    let db: Database = client.database("Ecom");
    let collection = db.collection("User");

    let repo = authrepo::new(collection);
    let service = Arc::new(authservice::new(repo));

    let app = Router::new()
        .nest("/auth", routes(service.clone()))
        .nest("/cart", routes(service.clone()))
        .nest("/category", routes(service.clone()))
        .nest("/inventory", routes(service.clone()))
        .nest("/orders", routes(service.clone()))
        .nest("/products", routes(service.clone()))
        .nest("/reviews", routes(service.clone()))
        .nest("/wishlist", routes(service.clone()))
        .nest("/users", routes(service.clone()));

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
