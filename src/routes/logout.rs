use crate::errors::Error;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/logout", get(logout))
}

async fn logout() -> Result<(), Error> {
    todo!();
}
