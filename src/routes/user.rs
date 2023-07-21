use crate::database::Database;
use crate::errors::Error;
use crate::server::context::Context;
use crate::utils;

use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

pub async fn routes(database: Arc<Database>) -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user_with_id).delete(delete_user).patch(update_user),
        )
        .with_state(database)
}

async fn create_user(
    State(database): State<Arc<Database>>,
    payload: Multipart,
) -> Result<Response, Error> {
    let mut user_info = utils::multipart::parse_user_for_create_from_multipart(payload).await?;
    let user_id = database.create_user(&mut user_info).await?;
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully created user.",
        },
        "user_id": user_id
    }));
    let res = (StatusCode::CREATED, body).into_response();

    Ok(res)
}

// #[axum_macros::debug_handler]
async fn list_users(
    _context: Context,
    State(database): State<Arc<Database>>,
) -> Result<Response, Error> {
    let users = database.get_all_users().await?;
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully get all user.",
        },
        "users": users
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

async fn get_user_with_id(
    _context: Context,
    State(database): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Response, Error> {
    let user = database.get_user_with_id(&id).await?;
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully get user.",
        },
        "user": user
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

// #[axum_macros::debug_handler]
async fn update_user(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<Response, Error> {
    let user_info = utils::multipart::parse_user_for_create_from_multipart(payload).await?;
    database
        .update_user_with_id(&id, &context, &user_info)
        .await?;
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully update user.",
        },
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

async fn delete_user(
    _context: Context,
    State(database): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Response, Error> {
    let user = database.delete_user_with_id(&id).await?;
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully delete user.",
        },
        "user": user
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}
