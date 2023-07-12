pub mod config;

use crate::errors::{s3::S3Error, Error};
use crate::s3::config::S3Config;

use s3::{bucket::Bucket, creds::Credentials, region::Region};

pub async fn get_bucket() -> Result<Bucket, Error> {
    let config = S3Config::parse_from_env_file("./.env")?;

    let credentials = Credentials {
        access_key: Some(config.user),
        secret_key: Some(config.password),
        security_token: None,
        session_token: None,
        expiration: None,
    };
    let region = Region::Custom {
        region: "ap-east-1".to_owned(),
        endpoint: format!("http://{}:{}", config.ip, config.api_port),
    };
    let bucket = Bucket::new(&config.bucket_name, region, credentials)
        .map_err(|err| {
            log::error!(
                "Could not instantiate bucket: {}.\n   --> Cause: {}",
                &config.bucket_name,
                err
            );
            S3Error::CouldNotInitBucket
        })?
        .with_path_style();

    Ok(bucket)
}
