use crate::controllers;
use crate::errors::Error;
use crate::models::user::User;
use crate::server::context::Context;

use axum::{
    extract::{Multipart, Path, State},
    routing::get,
    Json, Router,
};
use serde_json::Value;
use std::sync::Arc;

pub async fn routes(context: Arc<Context>) -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user_with_id).delete(delete_user).put(update_user),
        )
        .with_state(context)
}

async fn create_user(
    context: State<Arc<Context>>,
    payload: Multipart,
) -> Result<Json<Value>, Error> {
    return controllers::user::create_user(context, payload).await;
}

// #[axum_macros::debug_handler]
async fn list_users(context: State<Arc<Context>>) -> Result<Json<Vec<User>>, Error> {
    return controllers::user::list_users(context).await;
}

async fn get_user_with_id(
    context: State<Arc<Context>>,
    id: Path<String>,
) -> Result<Json<User>, Error> {
    return controllers::user::get_user_with_id(context, id).await;
}

// #[axum_macros::debug_handler]
async fn update_user(
    context: State<Arc<Context>>,
    id: Path<String>,
    payload: Multipart,
) -> Result<Json<Value>, Error> {
    return controllers::user::update_user(context, id, payload).await;
}

// #[axum_macros::debug_handler]
async fn delete_user(context: State<Arc<Context>>, id: Path<String>) -> Result<Json<User>, Error> {
    return controllers::user::delete_user(context, id).await;
}
