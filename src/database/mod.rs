pub mod config;
pub mod create_tables;

use crate::database::config::DatabaseConfig;
use crate::errors::{database::DBError, Error};
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
    let db = Surreal::new::<Ws>(config.address).await.map_err(|error| {
        log::error!("Couldn't connect to database.\n    --> Cause: {}", error);
        DBError::CouldNotOpenWebSocket
    })?;
    log::info!("Successfully connected to database server");

    log::info!("Attempting to log in to database server");
    db.signin(Root {
        username: &config.username,
        password: &config.password,
    })
    .await
    .map_err(|error| {
        log::error!("Couldn't connect to database.\n    --> Cause: {}", error);
        DBError::AuthFailed
    })?;
    log::info!("Successfully logged in to database server");

    db.use_ns(config.namespace.clone())
        .use_db(config.database.clone())
        .await
        .map_err(|error| {
            log::error!(
                "Couldn't connect to namespace: {}, in database: {}.\n    --> Cause: {}",
                config.namespace,
                config.database,
                error
            );
            DBError::CouldNotConnectToNameSpace
        })?;
    log::info!(
        "Successfully connected to database with: {{ namespace: {}, database: {} }}",
        config.namespace,
        config.database
    );

    Ok(db)
}
