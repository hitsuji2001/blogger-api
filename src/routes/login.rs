use crate::auth::{jwt, jwt::Role};
use crate::models::user::{User, UserForLogin, USER_TBL_NAME};
use crate::server::context::Context;
use crate::Error;

use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn routes(context: Arc<Context>) -> Router {
    return Router::new()
        .route("/login", post(login))
        .with_state(context);
}

#[axum_macros::debug_handler]
async fn login(
    State(context): State<Arc<Context>>,
    payload: Json<UserForLogin>,
) -> Result<Json<Value>, Error> {
    let sql = format!(
        "SELECT * FROM {} WHERE email == '{}'",
        USER_TBL_NAME, payload.email
    );
    let user: Vec<User> = context
        .database
        .query(sql)
        .await
        .map_err(|err| Error::DBCouldNotSelectUser(payload.email.clone(), err.to_string()))?
        .take(0)
        .map_err(|err| Error::DBCouldNotSelectUser(payload.email.clone(), err.to_string()))?;

    if user.len() == 0 {
        return Err(Error::ServerCouldNotAuthenticateUser);
    } else if user.len() > 1 {
        return Err(Error::ServerDuplicateUserEmail);
    }
    let user = &user[0];
    let token = jwt::create_jwt(&user.id, &Role::User)?;

    let response = Json(json!({
        "result": {
            "success": true,
        },
        "token": format!("{}", token),
    }));

    Ok(response)
}
