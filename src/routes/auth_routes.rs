
use axum::{
    debug_handler, extract::{Query, State}, http::StatusCode, response::Redirect, routing::get, Router
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::Deserialize;
use reqwest::Client;
// use jsonwebtoken::{decode, DecodingKey, Validation};
use std::collections::HashMap;
use crate::state::{AppState, AuthState};

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: String,
    refresh_token: Option<String>,
}

#[derive(Deserialize)]
struct OAuthCallbackParams {
    code: String,
}


#[debug_handler]
async fn oauth_callback(
    Query(params): Query<OAuthCallbackParams>, 
    jar: CookieJar,
    State(auth): State<AuthState>,
) -> Result<(CookieJar, Redirect), StatusCode> {
    tracing::info!("handling code {:?}", params.code);
    let token_response = exchange_code_for_token(&params.code, auth).await;


    if token_response.is_ok() {
        let tokens = token_response.unwrap();
        let cookie = Cookie::build(("auth_token", tokens.id_token))
            .http_only(true)  // Ensure it's an HTTP-only cookie
            .same_site(SameSite::Lax)
            .secure(true)  // Only send cookie over HTTPS
            .path("/")
            .build();
        return Ok((jar.add(cookie), Redirect::to("/")));
    }
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
}

async fn exchange_code_for_token(code: &str, auth_state: AuthState) -> Result<TokenResponse, reqwest::Error> {
    let client = Client::new();

    let params = HashMap::from([
        ("grant_type", "authorization_code"),
        ("client_id", auth_state.client_id.as_str()),
        ("client_secret", auth_state.client_secret.as_str()),
        ("code", code),
        ("redirect_uri", auth_state.redirect_uri.as_str()),
    ]);

    let url = format!("{}/oauth2/token", auth_state.domain);
    let response = client
        .post(url)
        .form(&params)
        .send()
        .await?;
    let token_response = response.json::<TokenResponse>().await?;
    tracing::info!("token_response: {:?}", token_response);

    Ok(token_response)
}

async fn login_redirect(State(auth): State<AuthState>) -> Redirect {
    let login_url = format!(
        "{}/login?response_type=code&client_id={}&redirect_uri={}",
        auth.domain, auth.client_id, auth.redirect_uri
    );
    Redirect::to(&login_url)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/callback", get(oauth_callback))
        .route("/login", get(login_redirect))
}
