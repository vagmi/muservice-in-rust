use async_trait::async_trait;
use axum::{http::request::Parts, extract::{FromRequestParts, FromRef}, Json};
use hyper::{StatusCode, header::AUTHORIZATION};
use jsonwebtoken::{decode, Validation, Algorithm, DecodingKey};
use serde::{Serialize, Deserialize};

use crate::app::DbApp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    name: String
}

pub struct ExtractAuthorization(Claims);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractAuthorization 
where 
    DbApp: FromRef<S>,
    S: Send + Sync 
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let db_app = DbApp::from_ref(state);
        if let Some(token) = parts.headers.get(AUTHORIZATION) {
            let jwt = token.to_str().map_err(|_| (StatusCode::BAD_REQUEST, "Token should be a string"))?;
            let data = decode::<Claims>(jwt, 
                                          &DecodingKey::from_secret(db_app.secret.as_bytes()), 
                                          &Validation::new(Algorithm::HS256))
                .map_err(|_e| {
                    (StatusCode::UNAUTHORIZED, "Unable to decode JWT")
                })?;
            return Ok(Self(data.claims))
        }
        Err((StatusCode::UNAUTHORIZED, "Unauthorized request"))
    }
}

pub async fn secure_endpoint(ExtractAuthorization(token): ExtractAuthorization) -> Json<Claims>{
    Json(token)
}
