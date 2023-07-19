use crate::auth;
use crate::errors::Error;

use axum::{http::Request, middleware::Next, response::Response};

pub async fn require_auth<T>(req: Request<T>, next: Next<T>) -> Result<Response, Error> {
    auth::jwt::authorize(req.headers()).await?;
    Ok(next.run(req).await)
}
