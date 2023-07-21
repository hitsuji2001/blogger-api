use crate::auth::{jwt, jwt::Role};
use crate::database::Database;
use crate::models::user::UserForLogin;
use crate::Error;

use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn routes(context: Arc<Database>) -> Router {
    return Router::new()
        .route("/login", post(login))
        .with_state(context);
}

#[axum_macros::debug_handler]
async fn login(
    State(database): State<Arc<Database>>,
    payload: Json<UserForLogin>,
) -> Result<Json<Value>, Error> {
    let user = database.get_user_with_email(&payload.email).await?;
    let token = jwt::create_jwt(&user.id, &Role::User)?;

    let response = Json(json!({
        "result": {
            "success": true,
        },
        "token": format!("{}", token),
    }));

    Ok(response)
}
