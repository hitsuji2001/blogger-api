use crate::database::{article::ARTICLE_TBL_NAME, user::USER_TBL_NAME, Database};
use crate::errors::Error;
use crate::server::context::Context;
use crate::utils;

use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use surrealdb::sql::Thing;

pub fn routes(database: Arc<Database>) -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:user_id",
            get(get_user_with_id).delete(delete_user).patch(update_user),
        )
        .route(
            "/users/:user_id/articles",
            get(list_articles).post(create_article),
        )
        .route(
            "/users/:user_id/articles/:article_id",
            patch(update_article),
        )
        .with_state(database.clone())
}

async fn create_user(
    State(database): State<Arc<Database>>,
    payload: Multipart,
) -> Result<Response, Error> {
    let mut user_info = utils::multipart::parse_user_for_create(payload).await?;
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
    context: Context,
    State(database): State<Arc<Database>>,
) -> Result<Response, Error> {
    context.check_permissions(None, true)?;

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
    context: Context,
    State(database): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Response, Error> {
    let id = Thing::from((USER_TBL_NAME, id.as_str()));
    context.check_permissions(Some(id.clone()), false)?;

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
    let id = Thing::from((USER_TBL_NAME, id.as_str()));
    context.check_permissions(Some(id.clone()), false)?;

    let user_info = utils::multipart::parse_user_for_create(payload).await?;
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
    context: Context,
    State(database): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Response, Error> {
    let id = Thing::from((USER_TBL_NAME, id.as_str()));
    context.check_permissions(Some(id.clone()), false)?;

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

async fn list_articles(
    context: Context,
    Path(user_id): Path<String>,
    State(database): State<Arc<Database>>,
) -> Result<Response, Error> {
    let user_id = Thing::from((USER_TBL_NAME, user_id.as_str()));
    context.check_permissions(Some(user_id), false)?;

    let articles = database.list_articles_for_user(&context).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": format!("Successfully list articles for user `{}`", context.user_id)
        },
        "articles": articles
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

#[axum_macros::debug_handler]
async fn create_article(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<Response, Error> {
    let id = Thing::from((USER_TBL_NAME, id.as_str()));
    context.check_permissions(Some(id), false)?;

    let mut article = utils::multipart::parse_article_for_create(payload, &context).await?;
    let article_id = database.create_article(&mut article).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully created articles.",
        },
        "article_id": article_id
    }));
    let res = (StatusCode::CREATED, body).into_response();

    Ok(res)
}

async fn update_article(
    context: Context,
    State(database): State<Arc<Database>>,
    Path((user_id, article_id)): Path<(String, String)>,
    payload: Multipart,
) -> Result<Response, Error> {
    let user_id = Thing::from((USER_TBL_NAME, user_id.as_str()));
    context.check_permissions(Some(user_id), false)?;

    let article = database
        .get_article_with_id(&Thing::from((ARTICLE_TBL_NAME, article_id.as_str())))
        .await?;
    utils::multipart::parse_article_for_update(payload, &article.article_uri).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully update article.",
        },
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}
