use async_graphql::{InputObject, SimpleObject};
use serde::{Serialize, Deserialize};

// --------------------- INPUT OBJECTS --------------------- //

#[derive(InputObject)]
pub struct SigninInput {
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct SignupInput {
    pub name: String,
    pub email: String,
    pub phone_number: i64,
    pub password: String,
}

#[derive(InputObject)]
pub struct GoogleCallbackInput {
    pub code: String,
}

#[derive(InputObject)]
pub struct UpdatePasswordInput {
    pub old_password: String,
    pub new_password: String,
}

#[derive(InputObject)]
pub struct UpdateProfileInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<i64>,
}

// --------------------- OUTPUT OBJECTS --------------------- //

#[derive(SimpleObject, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone_number: i64,
    pub created_at: String,
}

#[derive(SimpleObject)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}
