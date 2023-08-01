pub mod article;
pub mod comment;
pub mod config;
pub mod event;
pub mod like;
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

        log::debug!("Attempting to log in to database server");
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
        log::info!("Successfully connected to database");

        self.create_all_table().await?;
        self.create_events().await?;

        Ok(())
    }

    async fn create_all_table(&self) -> Result<(), Error> {
        self.create_user_table().await?;
        self.create_comment_table().await?;
        self.create_article_table().await?;
        self.create_like_table().await?;

        Ok(())
    }
}
