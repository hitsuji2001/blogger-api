use crate::errors::{database::DBError, Error};
use crate::models::user::{User, UserForCreate};
use crate::utils;

use axum::{
    extract::{Multipart, Path, State},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str;
use surrealdb::{engine::remote::ws::Client, opt::PatchOp, Surreal};

const USER_TBL_NAME: &str = "user";
const USER_PROFILE_FOLDER: &str = "/user_profile_pictures";

pub async fn create_user(
    State(db): State<Surreal<Client>>,
    payload: Multipart,
) -> Result<Json<Value>, Error> {
    log::info!("Controller::create_user");
    let mut user = utils::multipart::parse_user_for_create_from_multipart(payload).await?;
    let body = if user.validate()? {
        Json(json!({
            "result": {
                "success": false,
                "message": "Provided email is not a valid",
            },
        }))
    } else {
        if let Some(user_avatar) = user.avatar_as_bytes {
            let file_name =
                utils::multipart::upload_to_s3(&USER_PROFILE_FOLDER.to_string(), &user_avatar)
                    .await?;

            user.profile_pic_uri = Some(file_name);
        };

        let user: User = db
            .create(USER_TBL_NAME)
            .content(UserForCreate::new(
                &user.first_name,
                &user.last_name,
                &user.email,
                &user.profile_pic_uri,
            ))
            .await
            .map_err(|err| {
                log::error!("Failed to create user.\n    --> Cause: {}", err);
                DBError::UserCreateFailed
            })?;
        log::debug!("user: {:?}", user);

        Json(json!({
            "result": {
                "success": true,
                "message": "Successfully created user.",
            },
        }))
    };

    Ok(body)
}

// #[axum_macros::debug_handler]
pub async fn list_users(State(db): State<Surreal<Client>>) -> Result<Json<Vec<User>>, Error> {
    let users: Vec<User> = db.select(USER_TBL_NAME).await.map_err(|err| {
        log::error!("Could not get all users.\n    --> Cause: {}", err);
        DBError::UserSelectFailed
    })?;

    log::debug!("Successfully get all users");
    Ok(Json(users))
}

pub async fn get_user_with_id(
    State(db): State<Surreal<Client>>,
    Path(id): Path<String>,
) -> Result<Json<User>, Error> {
    let user: User = db.select((USER_TBL_NAME, &id)).await.map_err(|err| {
        log::error!(
            "Could not get user with id: {}.\n    --> Cause: {}",
            &id,
            err
        );
        DBError::UserSelectFailed
    })?;
    log::debug!("Successfully get user with id: {}. user: {:?}", &id, &user);

    Ok(Json(user))
}

#[axum_macros::debug_handler]
pub async fn update_user(
    State(db): State<Surreal<Client>>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<Json<Value>, Error> {
    let user = utils::multipart::parse_user_for_create_from_multipart(payload).await?;
    let mut file_path = String::default();
    if let Some(user_avatar) = user.avatar_as_bytes {
        file_path =
            utils::multipart::upload_to_s3(&USER_PROFILE_FOLDER.to_string(), &user_avatar).await?;
    };
    let changes: Vec<OpChanges> = db
        .update((USER_TBL_NAME, &id))
        .patch(PatchOp::replace("/updated_at", chrono::offset::Utc::now()))
        .patch(PatchOp::replace("/first_name", &user.first_name))
        .patch(PatchOp::replace("/last_name", &user.last_name))
        .patch(PatchOp::replace("/profile_pic_uri", &file_path))
        .await
        .map_err(|err| {
            log::error!(
                "Could not update user with id: {}.\n    --> Cause: {}",
                &id,
                err
            );
            DBError::UserUpdateFailed
        })?;

    log::debug!(
        "Successfully updated user with id: `{}`, changes: {:?}",
        &id,
        changes
    );
    Ok(Json(json!({
            "result": {
                "success": true,
                "message": "Successfully updated user.",
            },
    })))
}

#[axum_macros::debug_handler]
pub async fn delete_user(
    State(db): State<Surreal<Client>>,
    Path(id): Path<String>,
) -> Result<Json<User>, Error> {
    let user: User = db.delete((USER_TBL_NAME, &id)).await.map_err(|err| {
        log::error!(
            "Could not delete user with id: {:?}\n    --> Cause: {}",
            &id,
            err
        );
        DBError::UserDeleteFailed
    })?;

    log::debug!(
        "Successfully deleted user with: id `{}`. user: {:?}",
        &id,
        &user
    );

    Ok(Json(user))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OpChanges {
    op: String,
    path: String,
    value: String,
}
