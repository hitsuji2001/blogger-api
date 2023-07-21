use crate::auth::jwt;
use crate::errors::Error;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

pub struct Context {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or(Error::ServerUnauthorizedUser)?;

        let id = jwt::authorize(&parts.headers).await?;

        Ok(Context { user_id: id })
    }
}
