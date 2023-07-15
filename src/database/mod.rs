pub mod config;
pub mod create_tables;

use crate::database::config::DatabaseConfig;
use crate::errors::Error;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub async fn start() -> Result<Surreal<Client>, Error> {
    let config = DatabaseConfig::parse_from_env_file(&String::from("./.env"))?;
    log::info!(
        "Connecting to database server at: http://{}",
        config.address
    );
    let db = Surreal::new::<Ws>(config.address.clone())
        .await
        .map_err(|error| Error::DBCouldNotOpenWebSocket(config.address, error.to_string()))?;
    log::info!("Successfully connected to database server");

    log::info!("Attempting to log in to database server");
    db.signin(Root {
        username: &config.username,
        password: &config.password,
    })
    .await
    .map_err(|error| Error::DBAuthenticationFailed(error.to_string()))?;
    log::info!("Successfully logged in to database server");

    db.use_ns(config.namespace.clone())
        .use_db(config.database.clone())
        .await
        .map_err(|error| {
            Error::DBCouldNotConnectToNamespace(config.namespace.clone(), error.to_string())
        })?;
    log::info!(
        "Successfully connected to database with: {{ namespace: {}, database: {} }}",
        &config.namespace,
        &config.database
    );
    create_all_table(&db).await?;

    Ok(db)
}

async fn create_all_table(db: &Surreal<Client>) -> Result<(), Error> {
    create_tables::user(&db).await?;
    create_tables::article(&db).await?;

    Ok(())
}
