use crate::database::Database;
use crate::routes;

use axum::Router;
use std::sync::Arc;

pub fn routes(database: Arc<Database>) -> Router {
    Router::new()
        .nest("/api", routes::logout::routes())
        .nest("/api", routes::healthz::routes())
        .nest("/api", routes::user::routes(database.clone()))
        .nest("/api", routes::login::routes(database.clone()))
        .nest("/api", routes::comment::routes(database.clone()))
        .nest("/api", routes::like::routes(database.clone()))
        .nest("/api", routes::article::routes(database))
}
