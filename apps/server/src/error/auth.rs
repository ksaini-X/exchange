use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AuthError {
    InvalidCredentials,
    InvalidPassword,
    EmailAlreadyExists,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid email or password")
            }
            AuthError::InvalidPassword => (
                StatusCode::BAD_REQUEST,
                "Password does not meet requirements",
            ),
            AuthError::EmailAlreadyExists => (StatusCode::CONFLICT, "Email already exists"),
        };

        (status, message).into_response()
    }
}
