use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,

    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Email already exists")]
    EmailExists,

    #[error("Token creation failed")]
    TokenCreationFailed,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Password hashing failed")]
    HashError,

    #[error("Unauthorized")]
    Unauthorized,
}
