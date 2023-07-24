use crate::database::{user::USER_TBL_NAME, Database};
use crate::errors::Error;
use crate::models::user::Role;
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
use surrealdb::sql::Thing;

pub fn routes(database: Arc<Database>) -> Router {
    Router::new()
        .route("/articles", get(list_articles).post(create_article))
        .route(
            "/articles/:article_id",
            get(get_article_with_id)
                .delete(delete_article)
                .patch(update_article),
        )
        .with_state(database)
}

#[axum_macros::debug_handler]
async fn create_article(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<Response, Error> {
    let id = Thing::from((USER_TBL_NAME, id.as_str()));
    if context.user_id != id && context.user_role != Role::Admin {
        return Err(Error::ServerPermissionDenied(String::from(
            "Could not access other user information",
        )));
    }

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

async fn list_articles(
    _context: Context,
    State(_database): State<Arc<Database>>,
) -> Result<Response, Error> {
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully list articles.",
        },
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

async fn get_article_with_id(
    _context: Context,
    State(_database): State<Arc<Database>>,
    Path(_id): Path<String>,
) -> Result<Response, Error> {
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully get article.",
        },
    }));
    let res = (StatusCode::CREATED, body).into_response();

    Ok(res)
}

async fn update_article(
    _context: Context,
    State(_database): State<Arc<Database>>,
    Path(_id): Path<String>,
    _payload: Multipart,
) -> Result<Response, Error> {
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully update article.",
        },
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

async fn delete_article(
    _context: Context,
    State(_database): State<Arc<Database>>,
    Path(_id): Path<String>,
) -> Result<Response, Error> {
    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully delete article.",
        },
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}
