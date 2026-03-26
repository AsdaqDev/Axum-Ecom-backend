use crate::bootstrap::bootstrap;
use axum::{Router, routing::get, Extension};
use std::sync::Arc;

//Routes
use crate::modules::auth::controller::routes as auth_routes;

#[tokio::main]
async fn main() {
    let state = Arc::new(bootstrap().await);
    let app = Router::new()
    .nest("/auth", auth_routes())
    .layer(Extension(state));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}