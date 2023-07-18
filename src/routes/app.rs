use crate::errors::Error;
use crate::routes;
use crate::server::context::Context;

use axum::Router;
use std::sync::Arc;

pub async fn routes(context: Arc<Context>) -> Result<Router, Error> {
    Ok(Router::new()
        .nest("/api", routes::healthz::routes())
        .nest("/api", routes::user::routes(context).await)
        .nest("/api", routes::auth::routes().await))
}
