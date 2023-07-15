use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    return Router::new().route("/healthz", get(healthcheck));
}

// TODO: Redefine this function to check for the `alive` state
//       of database server and object storage server
//       and not just this server
async fn healthcheck() -> Result<Json<Value>, ()> {
    log::info!("Handler::healthcheck");
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "ok",
        }
    }));

    Ok(body)
}
