use crate::middlewares;
use crate::routes;
use crate::server::context::Context;

use axum::{middleware, Router};
use std::sync::Arc;

pub async fn routes(context: Arc<Context>) -> Router {
    Router::new()
        .nest("/api", routes::user::routes(context.clone()).await)
        .nest("/api", routes::logout::routes())
        .route_layer(middleware::from_fn(middlewares::auth::require_auth))
        .nest("/api", routes::healthz::routes())
        .nest("/api", routes::login::routes(context))
}
