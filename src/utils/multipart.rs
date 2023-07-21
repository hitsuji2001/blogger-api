use crate::errors::Error;
use crate::models::user::UserForCreate;
use crate::s3;
use crate::utils::image::{Image, ImageType};

use axum::{body::Bytes, extract::Multipart};

// TODO: Find a better way to parse multipart form to struct
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
            let file_name = field.file_name();
            let file_type = field.content_type();

            if name == "avatar" {
                user.avatar = Some(Image::new());
                let mut avatar = user
                    .avatar
                    .as_mut()
                    .expect("Unreachable, avatar should be contructed by now");
                if let Some(name) = file_name {
                    avatar.file_name = name.to_string();
                }
                if let Some(file_type) = file_type {
                    avatar.file_type = ImageType::from_str(file_type);
                }
                avatar.data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?
                    .to_vec();
            } else if name == "first_name" {
                let data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;
                user.first_name = parse_string_from_u8(&data)?;
            } else if name == "last_name" {
                let data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;
                user.last_name = parse_string_from_u8(&data)?;
            } else if name == "email" {
                let data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;
                user.email = parse_string_from_u8(&data)?;
            } else if name == "username" {
                let data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;
                user.username = parse_string_from_u8(&data)?;
            } else if name == "is_admin" {
                let data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;
                user.is_admin = parse_string_from_u8(&data)?
                    .parse::<bool>()
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;
            }
        }
    }

    Ok(user)
}

fn parse_string_from_u8(data: &Bytes) -> Result<String, Error> {
    let result = std::str::from_utf8(&data)
        .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?
        .to_string();

    Ok(result)
}

pub async fn upload_user_image_to_s3(
    user: &UserForCreate,
    base_folder: &str,
) -> Result<String, Error> {
    if let Some(avatar) = &user.avatar {
        let file_name = format!(
            "{}/{}.{}",
            base_folder,
            sha256::digest(format!(
                "{}/{}",
                avatar.file_name,
                chrono::offset::Utc::now()
            ))
            .get(0..32)
            .expect("Unreachable, SHA-256 should provide more than 32 chracter"),
            avatar.file_type.to_string()
        );
        log::info!("Uploading file: `{}` to s3.", &file_name);
        s3::get_bucket()
            .await?
            .put_object(&file_name, &avatar.data)
            .await
            .map_err(|err| Error::MinioCouldNotPutObject(err.to_string()))?;
        log::info!("Successfully uploaded file: `{}` to s3", &file_name);

        Ok(file_name)
    } else {
        return Err(Error::ServerCouldNotParseForm(String::from(
            "Unreachable, User avatar is null",
        )));
    }
}
