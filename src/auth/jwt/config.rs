use crate::errors::Error;

pub struct JWTConfig {
    pub secret: Vec<u8>,
    pub expriation: i64,
}

impl JWTConfig {
    pub fn parse_from_env_file() -> Result<Self, Error> {
        Ok(JWTConfig {
            secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set")
                .into_bytes(),
            expriation: std::env::var("JWT_EXPIRES_IN")
                .expect("JWT_EXPIRES_IN must be set")
                .parse::<i64>()
                .map_err(|error| Error::ParseEnvFailedWrongFormat(error.to_string()))?,
        })
    }
}
