use axum::{debug_handler, response::IntoResponse, routing::get, Router, extract::Json};
mod auth_routes;

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

pub fn create_router() -> Router {
    Router::new()
        .nest("/auth", auth_routes::router())
        .route("/api/index", get(index_route))
        .route("/api/about", get(about_route))
        .with_state(crate::state::AppState::new())
}
