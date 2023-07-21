use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;

pub fn routes() -> Router {
    return Router::new().route("/healthz", get(healthcheck));
}

// TODO: Redefine this function to check for the `alive` state
//       of database server and object storage server
//       and not just this server
async fn healthcheck() -> Result<Response, ()> {
    log::info!("Handler::healthcheck");
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "ok",
        }
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}
