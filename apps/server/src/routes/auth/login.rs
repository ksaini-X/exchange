use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::auth::AuthError;

#[derive(Deserialize)]
pub struct LoginRequestData {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponseData {
    token: String,
}

pub async fn login(
    Json(payload): Json<LoginRequestData>,
) -> Result<Json<LoginResponseData>, AuthError> {
    println!("Username : {}", payload.username);
    println!("Password : {}", payload.password);
    Ok(Json(LoginResponseData {
        token: String::from("abc"),
    }))
}
