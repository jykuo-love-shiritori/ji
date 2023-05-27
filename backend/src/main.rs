use std::net::SocketAddr;

use axum::{
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

mod handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(handler::hello_json))
        .nest_service("/", get_service(ServeDir::new("../frontend/dist")));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
