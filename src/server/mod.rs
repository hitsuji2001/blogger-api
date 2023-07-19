pub mod config;
pub mod context;

use crate::database;
use crate::errors::Error;
use crate::routes;
use crate::server::{config::ServerConfig, context::Context};

use axum::Router;
use std::sync::Arc;

async fn get_all_routes() -> Result<Router, Error> {
    let database = database::start().await?;
    let context = Arc::new(Context {
        database: database,
        user: Default::default(),
    });
    let routers = Router::new().merge(routes::app::routes(context).await);

    Ok(routers)
}

pub async fn start() -> Result<(), Error> {
    let config = ServerConfig::parse_from_env_file()?;

    log::info!("Server listening on http://{:?}", config.address);
    axum::Server::bind(&config.address)
        .serve(get_all_routes().await?.into_make_service())
        .await
        .map_err(|error| Error::ServerCouldNotStart(error.to_string()))?;
    Ok(())
}
