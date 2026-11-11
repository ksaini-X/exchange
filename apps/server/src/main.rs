use axum::Router;
use tokio::net::TcpListener;

use crate::routes::auth_router;

pub mod error;
pub mod routes;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let app = Router::new().nest("/auth", auth_router());

    axum::serve(listener, app).await.unwrap();
}
