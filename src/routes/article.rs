use crate::database::{article::ARTICLE_TBL_NAME, user::USER_TBL_NAME, Database};
use crate::errors::Error;
use crate::server::context::Context;

use axum::{
    extract::{Path, State},
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
        .route(
            "/articles/:article_id",
            get(get_article_with_id).delete(delete_article),
        )
        .with_state(database)
}

async fn get_article_with_id(
    State(database): State<Arc<Database>>,
    Path(article_id): Path<String>,
) -> Result<Response, Error> {
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
