use crate::models::user::User;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub struct Context {
    pub database: Surreal<Client>,
    pub user: Option<User>,
}
