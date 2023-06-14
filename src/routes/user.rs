use crate::database;
use crate::errors::{database::DBError, server::ServerError, Error};
use crate::models::user::User;
use crate::utils::db_record::Record;

use axum::{extract::State, routing::get, Json, Router};
use regex::Regex;
use serde::Deserialize;
use serde_json::{json, Value};
use surrealdb::{engine::remote::ws::Client, Surreal};

const USER_TBL_NAME: &str = "user";

pub async fn routes() -> Result<Router, Error> {
    let database = database::start().await?;

    Ok(Router::new()
        .route("/users", get(list_users).post(create_user))
        .with_state(database))
}

async fn create_user(
    State(db): State<Surreal<Client>>,
    payload: Json<UserPayload>,
) -> Result<Json<Value>, Error> {
    log::info!("Handler::create_user");
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .map_err(|err| {
        log::error!("Could not parse email regex.\n    -> Cause: {}", err);
        ServerError::InvalidRegex
    })?;

    let body = if !email_regex.is_match(&payload.email) {
        Json(json!({
            "result": {
                "success": false,
                "message": "Provided email is not a valid",
            },
        }))
    } else {
        let _created: Record = db
            .create(USER_TBL_NAME)
            .content(User::new(
                payload.first_name.clone(),
                payload.last_name.clone(),
                payload.email.clone(),
            ))
            .await
            .map_err(|err| {
                log::error!("Failed to create user.\n    --> Cause: {}", err);
                DBError::UserCreateFailed
            })?;

        Json(json!({
            "result": {
                "success": true,
                "message": "Successfully created user.",
            },
        }))
    };
    log::debug!("{:#?}", payload);

    Ok(body)
}

// #[axum_macros::debug_handler]
async fn list_users(State(db): State<Surreal<Client>>) -> Result<(), Error> {
    let users: Vec<User> = db.select(USER_TBL_NAME).await.map_err(|err| {
        log::error!("Could not get all users.\n    --> Cause: {}", err);
        DBError::UserSelectFailed
    })?;

    log::debug!("users = {:#?}", Json(users));

    Ok(())
}

#[derive(Debug, Deserialize)]
struct UserPayload {
    first_name: String,
    last_name: String,
    email: String,
}
