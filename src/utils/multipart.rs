use crate::errors::Error;
use crate::models::user::UserForCreate;
use crate::s3;

use axum::{body::Bytes, extract::Multipart};
use std::str;

pub async fn parse_user_for_create_from_multipart(
    mut payload: Multipart,
) -> Result<UserForCreate, Error> {
    let mut user = UserForCreate::new();
    while let Some(field) = payload
        .next_field()
        .await
        .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?
    {
        if let Some(field_name) = field.name() {
            let name = field_name.to_string();
            let data = field
                .bytes()
                .await
                .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;
            if name == "first_name" {
                user.first_name = parse_string_from_u8(&data)?;
            } else if name == "last_name" {
                user.last_name = parse_string_from_u8(&data)?;
            } else if name == "email" {
                user.email = parse_string_from_u8(&data)?;
            } else if name == "avatar" && data.len() != 0 {
                user.avatar_as_bytes = Some(data.to_vec());
            } else if name == "username" {
                user.username = parse_string_from_u8(&data)?;
            }
        }
    }

    Ok(user)
}

fn parse_string_from_u8(data: &Bytes) -> Result<String, Error> {
    let result = str::from_utf8(&data)
        .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?
        .to_string();

    Ok(result)
}

// TODO: Add user context to place file of user in their own directory
//       Ex: user_id/profile_pic/pic.png, user_id/blog_pic/pic.png
// TODO: Find a way to prevent naming conflict when upload to s3
pub async fn upload_to_s3(base_folder: &str, bytes: &Vec<u8>) -> Result<String, Error> {
    let file_name = format!("{}/{}", base_folder, chrono::offset::Utc::now());
    log::info!("Uploading file: `{}` to s3.", &file_name);
    s3::get_bucket()
        .await?
        .put_object(&file_name, &bytes)
        .await
        .map_err(|err| Error::MinioCouldNotPutObject(err.to_string()))?;
    log::info!("Successfully uploaded file: `{}` to s3", &file_name);

    Ok(file_name)
}
