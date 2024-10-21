use axum::serve;
use tokio::net::TcpListener;
mod routes;
mod state;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("0.0.0.0:4444").await.unwrap();
    let app = routes::create_router().await;
    tracing::info!("Listening on: localhost:4444");
    serve(listener, app).await.unwrap();
}
