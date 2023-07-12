use crate::errors::{s3::S3Error, server::ServerError, Error};
use crate::models::user::UserForCreate;
use crate::s3;

use axum::{body::Bytes, extract::Multipart};
use std::str;

pub async fn parse_user_for_create_from_multipart(
    mut payload: Multipart,
) -> Result<UserForCreate, Error> {
    let mut user = UserForCreate::default();
    while let Some(field) = payload.next_field().await.map_err(|err| {
        log::error!("Received no data.\n    --> More info: {}", err);
        ServerError::CouldNotParseUserForm
    })? {
        if let Some(field_name) = field.name() {
            let name = field_name.to_string();
            let data = field.bytes().await.map_err(|err| {
                log::error!(
                    "Could not parse data from field: `{}` in form.\n    --> Cause: {}",
                    &name,
                    err
                );
                ServerError::CouldNotParseUserForm
            })?;
            if name == "first_name" {
                user.first_name = parse_string_from_u8(&data)?;
            } else if name == "last_name" {
                user.last_name = parse_string_from_u8(&data)?;
            } else if name == "email" {
                user.email = parse_string_from_u8(&data)?;
            } else if name == "avatar" && data.len() != 0 {
                user.avatar_as_bytes = Some(data.to_vec());
            }
        }
    }

    Ok(user)
}

fn parse_string_from_u8(data: &Bytes) -> Result<String, Error> {
    let result = str::from_utf8(&data)
        .map_err(|err| {
            log::error!("Could not parse data into string.\n    --> Cause: {}", err);
            ServerError::CouldNotParseUserForm
        })?
        .to_string();

    Ok(result)
}

pub async fn upload_to_s3(base_folder: &str, bytes: &Vec<u8>) -> Result<String, Error> {
    let file_name = format!("{}/{}", base_folder, chrono::offset::Utc::now());
    log::info!("Uploading file: `{}` to s3.", &file_name);
    s3::get_bucket()
        .await?
        .put_object(&file_name, &bytes)
        .await
        .map_err(|err| {
            log::error!(
                "Could not upload file: `{}` to s3.\n    --> Cause: {}",
                &file_name,
                err
            );
            S3Error::CouldNotPutObject
        })?;
    log::info!("Successfully uploaded file: `{}` to s3", &file_name);

    Ok(file_name)
}
