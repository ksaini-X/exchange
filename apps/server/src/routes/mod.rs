use axum::{Router, routing::post};

use crate::routes::auth::{login::login, register::register};

pub mod auth;

pub fn auth_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
