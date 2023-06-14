use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    return Router::new().route("/healthcheck", get(healthcheck));
}

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
