use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::auth::AuthError;

#[derive(Deserialize)]
pub struct RegisterRequestData {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponseData {
    token: String,
}

pub async fn register(
    Json(payload): Json<RegisterRequestData>,
) -> Result<Json<RegisterResponseData>, AuthError> {
    println!("Username : {}", payload.username);
    println!("Password : {}", payload.password);
    Ok(Json(RegisterResponseData {
        token: String::from("abc"),
    }))
}
