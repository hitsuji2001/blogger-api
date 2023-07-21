pub mod config;
pub mod context;

use crate::database::Database;
use crate::errors::Error;
use crate::routes;
use crate::server::config::ServerConfig;

use axum::Router;

async fn get_all_routes() -> Result<Router, Error> {
    let mut database = Database::new();
    database.start().await?;
    let routers = Router::new().merge(routes::app::routes(database.into()).await);

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
