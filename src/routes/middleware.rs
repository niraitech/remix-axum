use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use axum_extra::extract::CookieJar;
use crate::state::{AuthState, Claims};
use reqwest::StatusCode;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub claims: Claims,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser 
where 
    AuthState: FromRef<S>,
    S: Send + Sync 
{
    type Rejection = (StatusCode, String);
    async fn from_request_parts(parts: &mut Parts, s: &S) -> Result<Self, Self::Rejection> {
        let auth = AuthState::from_ref(s);
        let jar =  CookieJar::from_headers(&parts.headers);
        if let Some(token) = jar.get("auth_token") {
            let claims = auth.decode(token.value())
                .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
            return Ok(Self { claims });
        }
        return Err((StatusCode::UNAUTHORIZED, "No auth token found".to_string()));
    }
}
