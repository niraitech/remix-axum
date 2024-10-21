use axum::extract::FromRef;

use std::env;
#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub auth: AuthState,
}
impl AppState {
    pub(crate) fn new() -> Self {
        Self { auth: AuthState::from_env() }
    }
}



#[derive(Debug, Clone)]
pub struct AuthState {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl AuthState {
    pub fn from_env() -> Self {
        let client_id= env::var("COGNITO_CLIENT_ID").unwrap();
        let client_secret= env::var("COGNITO_CLIENT_SECRET").unwrap();
        let domain = env::var("COGNITO_DOMAIN").unwrap();
        let redirect_uri = env::var("REDIRECT_URI").unwrap();
        Self {
            client_id,
            client_secret,
            domain,
            redirect_uri
        }
    }
}
