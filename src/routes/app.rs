use crate::errors::Error;
use crate::routes;
use axum::Router;

pub async fn routes() -> Result<Router, Error> {
    Ok(Router::new()
        .nest("/api", routes::healthcheck::routes())
        .nest("/api", routes::user::routes().await?))
}
