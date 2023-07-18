use surrealdb::{engine::remote::ws::Client, Surreal};

pub struct Context {
    pub db: Surreal<Client>,
}
