use crate::errors::{env::EnvError, Error};
use crate::utils::env;

pub struct S3Config {
    pub ip: String,
    pub bucket_name: String,
    pub console_port: i32,
    pub api_port: i32,
    pub user: String,
    pub password: String,
    pub https: bool,
}

impl S3Config {
    pub fn parse_from_env_file(file_path: &str) -> Result<Self, Error> {
        let env = env::get_env_parser_from_file(file_path)?;

        Ok(S3Config {
            ip: env::find_key_from_parser(&String::from("MINIO_IP"), &env)?,
            bucket_name: env::find_key_from_parser(&String::from("MINIO_BUCKET_NAME"), &env)?,
            console_port: env::find_key_from_parser(&String::from("MINIO_CONSOLE_PORT"), &env)?
                .parse::<i32>()
                .map_err(|error| {
                    log::error!("Could not parse env file.\n    --> Cause: {}", error);
                    EnvError::WrongFormat
                })?,
            api_port: env::find_key_from_parser(&String::from("MINIO_API_PORT"), &env)?
                .parse::<i32>()
                .map_err(|error| {
                    log::error!("Could not parse env file.\n    --> Cause: {}", error);
                    EnvError::WrongFormat
                })?,
            user: env::find_key_from_parser(&String::from("MINIO_ROOT_USER"), &env)?,
            password: env::find_key_from_parser(&String::from("MINIO_ROOT_PASSWORD"), &env)?,
            https: if env::find_key_from_parser(&String::from("MINIO_HTTPS"), &env)? == "false" {
                false
            } else {
                true
            },
        })
    }
}
