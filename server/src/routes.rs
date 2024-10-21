use axum::{debug_handler, response::IntoResponse, routing::get, Router, extract::Json};
use common_types::Message;
mod auth_routes;
mod user_routes;
mod middleware;


#[debug_handler]
async fn index_route() -> impl IntoResponse {
    Json(Message{message: "Hello, Axum!".to_string()})
}

#[debug_handler]
async fn about_route() -> impl IntoResponse {
    Json(Message{message: "Hello, About!".to_string()})
}

pub async fn create_router() -> Router {
    let state = crate::state::AppState::new().await;
    Router::new()
        .nest("/auth", auth_routes::router())
        .nest("/api/users", user_routes::router())
        .route("/api/index", get(index_route))
        .route("/api/about", get(about_route))
        .with_state(state)
}
