pub mod config;

use crate::auth::jwt::config::JWTConfig;
use crate::errors::Error;
use crate::models::user::Role;

use axum::http::HeaderMap;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

const AUTHORIZATION: &str = "Authorization";
const BEARER: &str = "Bearer ";

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn create_jwt(user: &Thing, role: &Role) -> Result<String, Error> {
    let config = JWTConfig::parse_from_env_file()?;

    let expriation = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(config.expriation))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user.to_string(),
        role: role.to_string(),
        exp: expriation as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let token = jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(&config.secret))
        .map_err(|err| Error::JWTTokenCreationError(err.to_string()))?;

    Ok(token)
}

pub async fn authorize(headers: &HeaderMap) -> Result<(Thing, String), Error> {
    let config = JWTConfig::parse_from_env_file()?;

    match parse_jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = jsonwebtoken::decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(&config.secret),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|err| Error::JWTTokenError(err.to_string()))?;
            let claim = decoded.claims.sub.split(":").collect::<Vec<&str>>();

            Ok((Thing::from((claim[0], claim[1])), decoded.claims.role))
        }
        Err(_) => {
            return Err(Error::ServerUnauthorizedUser);
        }
    }
}

fn parse_jwt_from_header(headers: &HeaderMap) -> Result<String, Error> {
    let header = match headers.get(AUTHORIZATION) {
        Some(value) => value,
        None => return Err(Error::JWTTokenNotFoundOnHeader),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(value) => value,
        Err(_) => return Err(Error::JWTTokenNotFoundOnHeader),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::JWTInvalidAuthHeader);
    }

    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
