use axum::{debug_handler, extract::State, response::IntoResponse, routing::get, Json, Router };
use common_types::CurrentUser;
use crate::state::{AppState, AuthState};

use super::middleware::AuthenticatedUser;


#[debug_handler]
async fn current_user(State(_auth): State<AuthState>, user: AuthenticatedUser) ->impl IntoResponse {
    Json(CurrentUser{email: user.claims.email.clone()})
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/current_user", get(current_user))
}
