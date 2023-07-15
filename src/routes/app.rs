use crate::errors::Error;
use crate::routes;
use axum::Router;

pub async fn routes() -> Result<Router, Error> {
    Ok(Router::new()
        .nest("/api", routes::healthz::routes())
        .nest("/api", routes::user::routes().await?)
        .nest("/api", routes::auth::routes().await))
}
