pub mod config;

use crate::database;
use crate::errors::Error;
use crate::routes;
use crate::server::config::ServerConfig;

use axum::Router;

async fn get_all_routes() -> Result<Router, Error> {
    Ok(Router::new().merge(routes::app::routes().await?))
}

pub async fn start() -> Result<(), Error> {
    let config = ServerConfig::parse_from_env_file("./.env")?;
    log::info!("Server listening on http://{:?}", config.address);
    let db = database::start().await?;
    database::create_tables::user(&db).await?;

    axum::Server::bind(&config.address)
        .serve(get_all_routes().await?.into_make_service())
        .await
        .map_err(|error| Error::ServerCouldNotStart(error.to_string()))?;
    Ok(())
}
