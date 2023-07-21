use crate::Error;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    return Router::new().route("/logout", get(logout));
}

async fn logout() -> Result<(), Error> {
    todo!();
}
