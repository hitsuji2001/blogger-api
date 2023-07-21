pub mod config;
pub mod create_tables;
pub mod user;

use crate::database::config::DatabaseConfig;
use crate::errors::Error;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            client: Surreal::init(),
        }
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let config = DatabaseConfig::parse_from_env_file()?;

        log::info!(
            "Connecting to database server at: http://{}",
            config.address
        );
        self.client = Surreal::new::<Ws>(config.address.clone())
            .await
            .map_err(|error| Error::DBCouldNotOpenWebSocket(config.address, error.to_string()))?;
        log::info!("Successfully connected to database server");

        log::info!("Attempting to log in to database server");
        self.client
            .signin(Root {
                username: &config.username,
                password: &config.password,
            })
            .await
            .map_err(|error| Error::DBAuthenticationFailed(error.to_string()))?;
        log::info!("Successfully logged in to database server");

        self.client
            .use_ns(config.namespace.clone())
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
        self.create_all_table().await?;

        Ok(())
    }

    async fn create_all_table(&self) -> Result<(), Error> {
        create_tables::user(&self.client).await?;
        create_tables::article(&self.client).await?;

        Ok(())
    }
}
