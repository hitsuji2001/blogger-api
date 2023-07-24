use crate::database::{article::ARTICLE_TBL_NAME, user::USER_TBL_NAME, Database};
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

async fn list_articles(
    context: Context,
    Path(id): Path<String>,
    State(database): State<Arc<Database>>,
) -> Result<Response, Error> {
    let id = Thing::from((USER_TBL_NAME, id.as_str()));
    context.check_permissions(Some(id), false)?;

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

async fn get_article_with_id(
    context: Context,
    State(database): State<Arc<Database>>,
    Path((_user_id, article_id)): Path<(String, String)>,
) -> Result<Response, Error> {
    context.check_permissions(None, false)?;

    let article = database
        .get_article_with_id(&Thing::from((ARTICLE_TBL_NAME, article_id.as_str())))
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully get article.",
        },
        "article": article
    }));
    let res = (StatusCode::CREATED, body).into_response();

    Ok(res)
}

async fn update_article(
    _context: Context,
    State(_database): State<Arc<Database>>,
    Path((_user_id, _article_id)): Path<(String, String)>,
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
    context: Context,
    State(database): State<Arc<Database>>,
    Path((user_id, article_id)): Path<(String, String)>,
) -> Result<Response, Error> {
    let user_id = Thing::from((USER_TBL_NAME, user_id.as_str()));
    context.check_permissions(Some(user_id), false)?;

    let article = database
        .delete_article_with_id(&Thing::from((ARTICLE_TBL_NAME, article_id.as_str())))
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully delete article.",
        },
        "article": article
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}
