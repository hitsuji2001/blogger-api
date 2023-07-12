use crate::controllers;
use crate::database;
use crate::errors::Error;
use crate::models::user::User;

use axum::{
    extract::{Multipart, Path, State},
    routing::get,
    Json, Router,
};
use serde_json::Value;
use surrealdb::{engine::remote::ws::Client, Surreal};

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

async fn create_user(db: State<Surreal<Client>>, payload: Multipart) -> Result<Json<Value>, Error> {
    return controllers::user::create_user(db, payload).await;
}

// #[axum_macros::debug_handler]
async fn list_users(db: State<Surreal<Client>>) -> Result<Json<Vec<User>>, Error> {
    return controllers::user::list_users(db).await;
}

async fn get_user_with_id(
    db: State<Surreal<Client>>,
    id: Path<String>,
) -> Result<Json<User>, Error> {
    return controllers::user::get_user_with_id(db, id).await;
}

#[axum_macros::debug_handler]
async fn update_user(
    db: State<Surreal<Client>>,
    id: Path<String>,
    payload: Multipart,
) -> Result<Json<Value>, Error> {
    return controllers::user::update_user(db, id, payload).await;
}

#[axum_macros::debug_handler]
async fn delete_user(db: State<Surreal<Client>>, id: Path<String>) -> Result<Json<User>, Error> {
    return controllers::user::delete_user(db, id).await;
}
