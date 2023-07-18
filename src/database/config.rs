use crate::errors::Error;

pub struct DatabaseConfig {
    pub address: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn parse_from_env_file() -> Result<Self, Error> {
        Ok(DatabaseConfig {
            address: format!(
                "{}:{}",
                std::env::var("DB_HOST").expect("DB_HOST must be set"),
                std::env::var("DB_PORT").expect("DB_PORT must be set"),
            ),
            username: std::env::var("DB_USER").expect("DB_USER must be set"),
            password: std::env::var("DB_PASS").expect("DB_PASS must be set"),
            namespace: std::env::var("DB_NAMESPACE").expect("DB_NAMESPACE must be set"),
            database: std::env::var("DB_DATABASE").expect("DB_DATABASE must be set"),
        })
    }
}
