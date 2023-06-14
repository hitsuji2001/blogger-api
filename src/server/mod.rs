pub mod config;

use crate::errors::{server::ServerError, Error};
use crate::routes;
use crate::server::config::ServerConfig;

use axum::Router;

async fn get_all_routes() -> Result<Router, Error> {
    Ok(Router::new().merge(routes::app::routes().await?))
}

pub async fn start() -> Result<(), Error> {
    let config = ServerConfig::parse_from_env_file("./.env")?;
    log::info!("Server listening on http://{:?}", config.address);

    axum::Server::bind(&config.address)
        .serve(get_all_routes().await?.into_make_service())
        .await
        .map_err(|error| {
            log::error!("[ERROR]: Couldn't start `web server`: {}", error);
            ServerError::CouldNotStartServer
        })?;
    Ok(())
}
