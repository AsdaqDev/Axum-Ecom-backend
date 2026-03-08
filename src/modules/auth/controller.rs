use axum::{
    extract::{json,state},
    routing::post,
    Router,
};

use super::modules::auth::dto::{
    SigninInput,
    SignupInput,
    GoogleCallbackInput,
    UpdatePasswordInput,
    UpdateProfileInput
};

pub fn routes() -> Router {
    Router::new()
    .route("auth/signin", post(signin))
    .route("auth/signup", post(signup))
    .route("auth/google/callback", post(google_callback))
    .route("auth/password/update", post(update_password))
    .route("auth/profile/update", post(update_profile))
}

pub async fn signin(){};
pub async fn signup(){};
pub fn google_callback(){};
pub fn update_password(){};
pub fn update_profile(){};