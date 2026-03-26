use axum::{
    extract::{json,state},
    routing::post,
    Router,
};
use std::sync::Arc;
use crate::modules::state::AppState;

use dto::{
    SigninInput,
    SignupInput,
    GoogleCallbackInput,
    UpdatePasswordInput,
    UpdateProfileInput
};

use modules::service::Authservice;

pub fn routes() -> Router {
    Router::new()
    .route("/auth/signup", post(signup))
    .route("/auth/signin", post(signin))
    .route("/auth/google/callback", post(google_callback))
    .route("/auth/password/update", post(update_password))
    .route("/auth/profile/update", post(update_profile))
}

pub async fn signup( State(state): State<Arc<AppState>>, Json(payload): Json<SignupInput>) Result<StatusCode,String>{
    let service = Authservice::new(&state.db)
    service.signup(payload).await?;
    Ok(StatusCode::CREATED)
}
pub async fn signin(){};
pub fn google_callback(){};
pub fn update_password(){};
pub fn update_profile(){};