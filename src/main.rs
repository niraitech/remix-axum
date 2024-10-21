use axum::{debug_handler, response::IntoResponse, routing::get, serve, Router, extract::Json};
use tokio::net::TcpListener;

#[derive(serde::Serialize, Debug)]
struct Message {
    message: String,
}

#[debug_handler]
async fn index_route() -> impl IntoResponse {
    Json(Message{message: "Hello, Axum!".to_string()})
}

#[debug_handler]
async fn about_route() -> impl IntoResponse {
    Json(Message{message: "Hello, About!".to_string()})
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("0.0.0.0:4444").await.unwrap();
    let app = Router::new()
        .route("/api/index", get(index_route))
        .route("/api/about", get(about_route));
    tracing::info!("Listening on: localhost:4444");
    serve(listener, app).await.unwrap();
}
