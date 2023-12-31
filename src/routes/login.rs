use crate::auth::jwt;
use crate::database::Database;
use crate::errors::Error;
use crate::models::user::{Role, UserForLogin};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

pub fn routes(context: Arc<Database>) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(context)
}

#[axum_macros::debug_handler]
async fn login(
    State(database): State<Arc<Database>>,
    payload: Json<UserForLogin>,
) -> Result<Response, Error> {
    let user = database.get_user_with_email(&payload.email).await?;
    let token = if !user.is_admin {
        jwt::create_jwt(&user.id, &Role::User)?
    } else {
        jwt::create_jwt(&user.id, &Role::Admin)?
    };

    let body = Json(json!({
        "result": {
            "success": true,
        },
        "token": format!("{}", token),
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}
