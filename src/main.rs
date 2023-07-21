mod auth;
mod database;
mod errors;
mod models;
mod routes;
mod s3;
mod server;
mod utils;

extern crate chrono;
extern crate dotenv;

use crate::errors::Error;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .expect("Couldn't create `simple logger`.");
    server::start().await?;
    Ok(())
}
