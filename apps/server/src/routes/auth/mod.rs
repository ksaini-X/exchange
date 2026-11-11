pub mod login;
pub mod register;

use crate::routes::auth::{login::login, register::register};
use axum::{Router, routing::post};

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
