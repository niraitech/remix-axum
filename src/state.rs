use axum::extract::FromRef;
use jsonwebtoken::{jwk::JwkSet, DecodingKey, Validation};
use serde::Deserialize;
use thiserror::Error;
use std::env;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub auth: AuthState,
}
impl AppState {
    pub(crate) async fn new() -> Self {
        Self { auth: AuthState::from_env().await.unwrap() }
    }
}


#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Unable to load JWKS {0}")]
    JWKSError(reqwest::Error),
    #[error("Error with JWT {0}")]
    JsonWebToken(jsonwebtoken::errors::Error),
    #[error("Provided JWT Token does not have a valid key id")]
    NoJWTKey,
}


#[derive(Debug, Clone)]
pub struct AuthState {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    issuer_url: String,
    jwks: JwkSet,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
}

impl AuthState {
    pub async fn from_env() -> Result<Self, AuthError> {
        let client_id= env::var("COGNITO_CLIENT_ID").unwrap();
        let client_secret= env::var("COGNITO_CLIENT_SECRET").unwrap();
        let domain = env::var("COGNITO_DOMAIN").unwrap();
        let issuer_url= env::var("COGNITO_ISSUER_URL").unwrap();
        let jwks_url= format!("{}/.well-known/jwks.json", issuer_url);
        let redirect_uri = env::var("REDIRECT_URI").unwrap();

        let resp = reqwest::get(&jwks_url).await.map_err(|r| AuthError::JWKSError(r))?;
        let jwks: JwkSet = resp.json().await.map_err(|r| AuthError::JWKSError(r))?;

        Ok(Self {
            client_id,
            client_secret,
            domain,
            redirect_uri,
            issuer_url,
            jwks
        })
    }
    pub fn decode(&self, token: &str) -> Result<Claims, AuthError> {
        let header = jsonwebtoken::decode_header(token).map_err(
            |e| AuthError::JsonWebToken(e)
        )?;
        let kid = header.kid.ok_or(AuthError::NoJWTKey)?;

        // Find the matching JWK for this token
        let jwk = self.jwks.find(&kid).ok_or(AuthError::NoJWTKey)?;

        // Get the public key from the JWK and verify the JWT
        let decoding_key = DecodingKey::from_jwk(jwk).map_err(|e| AuthError::JsonWebToken(e))?;

        // Set up validation options (e.g., algorithms, issuer)
        let mut validation = Validation::new(header.alg);
        validation.set_audience(&[self.client_id.clone()]);
        validation.set_issuer(&[self.issuer_url.clone()]);

        // Verify the token and return the claims
        let decoded_token = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|err| AuthError::JsonWebToken(err))?;

        Ok(decoded_token.claims)

    }
}
