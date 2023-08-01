use crate::database::{article::ARTICLE_TBL_NAME, comment::COMMENT_TBL_NAME, Database};
use crate::errors::Error;
use crate::server::context::Context;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use surrealdb::sql::Thing;

pub fn routes(database: Arc<Database>) -> Router {
    Router::new()
        .route("/like/comment/:comment_id", post(like_comment))
        .route("/like/article/:article_id", post(like_article))
        .route("/unlike/comment/:comment_id", post(unlike_comment))
        .route("/unlike/article/:article_id", post(unlike_article))
        .with_state(database)
}

async fn like_comment(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(comment_id): Path<String>,
) -> Result<Response, Error> {
    let comment_id = Thing::from((COMMENT_TBL_NAME, comment_id.as_str()));
    let like_id = database
        .like_comment_or_article(&context, &comment_id)
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully create like"
        },
        "like_id": like_id
    }));
    let res = (StatusCode::CREATED, body).into_response();

    Ok(res)
}

async fn like_article(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(article_id): Path<String>,
) -> Result<Response, Error> {
    let article_id = Thing::from((ARTICLE_TBL_NAME, article_id.as_str()));
    let like_id = database
        .like_comment_or_article(&context, &article_id)
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully create like"
        },
        "like_id": like_id
    }));
    let res = (StatusCode::CREATED, body).into_response();

    Ok(res)
}

async fn unlike_comment(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(comment_id): Path<String>,
) -> Result<Response, Error> {
    let comment_id = Thing::from((COMMENT_TBL_NAME, comment_id.as_str()));
    database
        .unlike_comment_or_article(&context, &comment_id)
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully remove like"
        },
    }));
    let res = (StatusCode::ACCEPTED, body).into_response();

    Ok(res)
}

async fn unlike_article(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(article_id): Path<String>,
) -> Result<Response, Error> {
    let article_id = Thing::from((ARTICLE_TBL_NAME, article_id.as_str()));
    database
        .unlike_comment_or_article(&context, &article_id)
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully remove like"
        },
    }));
    let res = (StatusCode::ACCEPTED, body).into_response();

    Ok(res)
}
