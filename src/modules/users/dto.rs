use serde::Serialize;
use async_graphql::SimpleObject;

#[derive(SimpleObject, Serialize, Clone)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(SimpleObject, Serialize, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}
