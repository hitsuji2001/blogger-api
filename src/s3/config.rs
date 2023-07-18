use crate::errors::Error;

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
    pub fn parse_from_env_file() -> Result<Self, Error> {
        Ok(S3Config {
            ip: std::env::var("MINIO_IP").expect("MINIO_IP must be set"),
            bucket_name: std::env::var("MINIO_BUCKET_NAME").expect("MINIO_BUCKET_NAME must be set"),
            console_port: std::env::var("MINIO_CONSOLE_PORT")
                .expect("MINIO_CONSOLE_PORT must be set")
                .parse::<i32>()
                .map_err(|error| Error::ParseEnvFailedWrongFormat(error.to_string()))?,
            api_port: std::env::var("MINIO_API_PORT")
                .expect("MINIO_API_PORT must be set")
                .parse::<i32>()
                .map_err(|error| Error::ParseEnvFailedWrongFormat(error.to_string()))?,
            user: std::env::var("MINIO_ROOT_USER").expect("MINIO_ROOT_USER must be set"),
            password: std::env::var("MINIO_ROOT_PASSWORD")
                .expect("MINIO_ROOT_PASSWORD must be set"),
            https: if std::env::var("MINIO_HTTPS").expect("MINIO_HTTPS must be set") == "false" {
                false
            } else {
                true
            },
        })
    }
}
