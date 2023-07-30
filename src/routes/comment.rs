use crate::database::{
    article::{ARTICLE_FOLDER, ARTICLE_TBL_NAME},
    comment::{COMMENT_FOLDER, COMMENT_TBL_NAME},
    Database,
};
use crate::errors::Error;
use crate::server::context::Context;
use crate::utils;

use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use surrealdb::sql::Thing;

pub fn routes(database: Arc<Database>) -> Router {
    Router::new()
        .route(
            "/comments/:comment_id",
            get(get_comment).put(update_comment).delete(delete_comment),
        )
        .route("/comments/reply/:comment_id", get(get_reply_for_comment))
        .with_state(database)
}

pub fn for_article_routes(database: Arc<Database>) -> Router {
    Router::new()
        .route("/comments", get(get_comment_for_article))
        .with_state(database)
}

pub fn for_user_routes(database: Arc<Database>) -> Router {
    Router::new()
        .route("/comments/:article_id", post(create_comment))
        .route("/reply/:article_id/:comment_id", post(create_reply))
        .with_state(database)
}

async fn get_comment(
    State(database): State<Arc<Database>>,
    Path(comment_id): Path<String>,
) -> Result<Response, Error> {
    let comment_id = Thing::from((COMMENT_TBL_NAME, comment_id.as_str()));
    let comment = database.get_comment(&comment_id).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully get comment"
        },
        "comment": comment
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

async fn get_reply_for_comment(
    State(database): State<Arc<Database>>,
    Path(comment_id): Path<String>,
) -> Result<Response, Error> {
    let comment_id = Thing::from((COMMENT_TBL_NAME, comment_id.as_str()));
    let reply = database.get_reply_for_comment(&comment_id).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": format!("Successfully get reply for comment with id: `{}`", comment_id)
        },
        "reply": reply
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

async fn get_comment_for_article(
    State(database): State<Arc<Database>>,
    Path(article_id): Path<String>,
) -> Result<Response, Error> {
    let article_id = Thing::from((ARTICLE_TBL_NAME, article_id.as_str()));
    let comments = database.get_comment_for_article(&article_id).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully get comments"
        },
        "comments": comments
    }));
    let res = (StatusCode::OK, body).into_response();

    Ok(res)
}

#[axum_macros::debug_handler]
async fn create_comment(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(article_id): Path<String>,
    payload: Multipart,
) -> Result<Response, Error> {
    context.check_permissions(None, false)?;
    let article_id = Thing::from((ARTICLE_TBL_NAME, article_id.as_str()));

    let mut comment =
        utils::multipart::parse_comment_for_create(payload, &context, &article_id).await?;
    let comment_id = database.create_comment(&mut comment).await?;

    if let Some(media) = comment.image {
        let uri = utils::multipart::upload_user_image_to_s3(
            format!(
                "{}/{}/{}/{}",
                context.user_id, ARTICLE_FOLDER, article_id, COMMENT_FOLDER
            )
            .as_str(),
            &media,
        )
        .await?;

        database.update_comment_uri(&comment_id, &uri).await?;
    }

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully create comment"
        },
        "comment_id": comment_id
    }));
    let res = (StatusCode::CREATED, body).into_response();

    Ok(res)
}

async fn create_reply(
    context: Context,
    State(database): State<Arc<Database>>,
    Path((article_id, comment_id)): Path<(String, String)>,
    payload: Multipart,
) -> Result<Response, Error> {
    let article_id = Thing::from((ARTICLE_TBL_NAME, article_id.as_str()));
    let comment_id = Thing::from((COMMENT_TBL_NAME, comment_id.as_str()));

    context.check_permissions(None, false)?;

    let mut comment =
        utils::multipart::parse_comment_for_create(payload, &context, &article_id).await?;
    let reply = database
        .create_reply(&context, &(article_id, comment_id), &mut comment)
        .await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully create reply"
        },
        "reply_id": reply
    }));
    let res = (StatusCode::CREATED, body).into_response();

    Ok(res)
}

async fn update_comment() -> Result<Response, Error> {
    todo!();
}

async fn delete_comment(
    context: Context,
    State(database): State<Arc<Database>>,
    Path(comment_id): Path<String>,
) -> Result<Response, Error> {
    let comment_id = Thing::from((COMMENT_TBL_NAME, comment_id.as_str()));
    let comment = database.get_comment(&comment_id).await?;
    context.check_permissions(Some(comment.user_id.clone()), false)?;
    database.delete_comment(&comment_id).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "message": "Successfully delete comment"
        },
    }));
    let res = (StatusCode::ACCEPTED, body).into_response();

    Ok(res)
}
