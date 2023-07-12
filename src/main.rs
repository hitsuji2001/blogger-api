mod controllers;
mod database;
mod errors;
mod models;
mod routes;
mod s3;
mod server;
mod utils;

use crate::errors::Error;
extern crate chrono;

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap_or_else(|error| {
            eprintln!(
                "[ERROR]: Couldn't create `simple logger`.\n    --> Cause: {}",
                error
            );
            panic!();
        });
    server::start().await?;
    Ok(())
}
