use crate::errors::Error;
use crate::utils::env;

pub struct DatabaseConfig {
    pub address: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn parse_from_env_file(file_path: &str) -> Result<Self, Error> {
        let env = env::get_env_parser_from_file(file_path)?;

        Ok(DatabaseConfig {
            address: format!(
                "{}:{}",
                env::find_key_from_parser(&String::from("DB_HOST"), &env)?,
                env::find_key_from_parser(&String::from("DB_PORT"), &env)?
            ),
            username: env::find_key_from_parser(&String::from("DB_USER"), &env)?,
            password: env::find_key_from_parser(&String::from("DB_PASS"), &env)?,
            namespace: env::find_key_from_parser(&String::from("DB_NAMESPACE"), &env)?,
            database: env::find_key_from_parser(&String::from("DB_DATABASE"), &env)?,
        })
    }
}
