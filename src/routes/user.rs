use crate::database;
use crate::errors::{database::DBError, server::ServerError, Error};
use crate::models::user::User;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use regex::Regex;
use serde::Deserialize;
use serde_json::{json, Value};
use surrealdb::{engine::remote::ws::Client, opt::PatchOp, Surreal};

const USER_TBL_NAME: &str = "user";

pub async fn routes() -> Result<Router, Error> {
    let database = database::start().await?;

    Ok(Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user_with_id).delete(delete_user).put(update_user),
        )
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
        let _user: User = db
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
async fn list_users(State(db): State<Surreal<Client>>) -> Result<Json<Vec<User>>, Error> {
    let users: Vec<User> = db.select(USER_TBL_NAME).await.map_err(|err| {
        log::error!("Could not get all users.\n    --> Cause: {}", err);
        DBError::UserSelectFailed
    })?;

    Ok(Json(users))
}

async fn get_user_with_id(
    State(db): State<Surreal<Client>>,
    Path(id): Path<String>,
) -> Result<Json<User>, Error> {
    let user: User = db
        .select((USER_TBL_NAME, id.clone()))
        .await
        .map_err(|err| {
            log::error!(
                "Could not get user with id: {}.\n    --> Cause: {}",
                id,
                err
            );
            DBError::UserSelectFailed
        })?;

    Ok(Json(user))
}

#[axum_macros::debug_handler]
async fn update_user(
    State(db): State<Surreal<Client>>,
    Path(id): Path<String>,
    payload: Json<UserUpdatePayload>,
) -> Result<Json<Value>, Error> {
    let _changes: Vec<OpChanges> = db
        .update((USER_TBL_NAME, id.clone()))
        //.patch(PatchOp::replace("/updated_at", chrono::offset::Utc::now()))
        .patch(PatchOp::replace("/first_name", payload.first_name.clone()))
        .patch(PatchOp::replace("/last_name", payload.last_name.clone()))
        .await
        .map_err(|err| {
            log::error!(
                "Could not update user with id: {}.\n    --> Cause: {}",
                id,
                err
            );
            DBError::UserUpdateFailed
        })?;

    Ok(Json(json!({
            "result": {
                "success": true,
                "message": "Successfully updated user.",
            },
    })))
}

#[axum_macros::debug_handler]
async fn delete_user(
    State(db): State<Surreal<Client>>,
    Path(id): Path<String>,
) -> Result<Json<User>, Error> {
    let user: User = db
        .delete((USER_TBL_NAME, id.clone()))
        .await
        .map_err(|err| {
            log::error!(
                "Could not delete user with id: {:?}\n    --> Cause: {}",
                id,
                err
            );
            DBError::UserDeleteFailed
        })?;

    Ok(Json(user))
}

#[derive(Debug, Deserialize)]
struct UserPayload {
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UserUpdatePayload {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OpChanges {
    op: String,
    path: String,
    value: String,
}
