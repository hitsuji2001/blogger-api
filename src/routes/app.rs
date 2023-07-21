use crate::database::Database;
use crate::routes;

use axum::Router;
use std::sync::Arc;

pub async fn routes(database: Arc<Database>) -> Router {
    Router::new()
        .nest("/api", routes::user::routes(database.clone()).await)
        .nest("/api", routes::logout::routes())
        .nest("/api", routes::healthz::routes())
        .nest("/api", routes::login::routes(database))
}
