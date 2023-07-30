use crate::errors::Error;
use crate::models::{article::ArticleForCreate, comment::CommentForCreate, user::UserForCreate};
use crate::s3;
use crate::server::context::Context;
use crate::utils::image::{Image, ImageType};

use axum::{body::Bytes, extract::Multipart};
use surrealdb::sql::Thing;

pub async fn parse_article_for_update(
    mut payload: Multipart,
    article_uri: &String,
) -> Result<(), Error> {
    while let Some(field) = payload
        .next_field()
        .await
        .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?
    {
        if let Some(field_name) = field.name() {
            let name = field_name.to_string();
            let file_type = field.content_type();
            if name == "file" {
                if let Some(file_type) = file_type {
                    if file_type != "text/html" {
                        return Err(Error::ServerUnsupportedMediaType(file_type.to_string()));
                    }
                }
                let data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;

                s3::get_bucket()
                    .await?
                    .put_object_with_content_type(article_uri, &data, "text/html")
                    .await
                    .map_err(|err| Error::MinioCouldNotPutObject(err.to_string()))?;
            }
        }
    }

    Ok(())
}

// TODO: Find a better way to parse multipart form to struct
pub async fn parse_user_for_create(mut payload: Multipart) -> Result<UserForCreate, Error> {
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
                    if !avatar.is_supported_image_type() {
                        return Err(Error::ServerUnsupportedMediaType(file_type.to_string()));
                    }
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
    let result = std::str::from_utf8(data)
        .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?
        .to_string();

    Ok(result)
}

pub async fn upload_user_image_to_s3(base_folder: &str, image: &Image) -> Result<String, Error> {
    let file_name = format!(
        "{}/{}.{}",
        base_folder,
        sha256::digest(format!(
            "{}/{}",
            image.file_name,
            chrono::offset::Utc::now()
        ))
        .get(0..32)
        .expect("Unreachable, SHA-256 should provide more than 32 chracter"),
        image.file_type
    );
    log::info!("Uploading file: `{}` to s3.", &file_name);
    s3::get_bucket()
        .await?
        .put_object(&file_name, &image.data)
        .await
        .map_err(|err| Error::MinioCouldNotPutObject(err.to_string()))?;
    log::info!("Successfully uploaded file: `{}` to s3", &file_name);

    Ok(file_name)
}

pub async fn parse_article_for_create(
    mut payload: Multipart,
    context: &Context,
) -> Result<ArticleForCreate, Error> {
    let mut article = ArticleForCreate::new();
    article.user_id = context.user_id.clone();

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
            if name == "title" {
                article.title = parse_string_from_u8(&data)?;
            }
        }
    }

    Ok(article)
}

pub async fn upload_html_to_s3(
    article: &ArticleForCreate,
    base_folder: &str,
) -> Result<String, Error> {
    let file_name = format!(
        "{}/{}.html",
        base_folder,
        sha256::digest(format!(
            "{}/{}/{}",
            article.user_id,
            article.title,
            chrono::offset::Utc::now()
        ))
        .get(0..32)
        .expect("Unreachable, SHA-256 should provide more than 32 chracter"),
    );

    log::info!("Uploading file: `{}` to s3.", &file_name);
    s3::get_bucket()
        .await?
        .put_object_with_content_type(
            &file_name,
            format!(
                "<!doctype html><html><head><title>{}</title></head><body><p>Placeholder</p></body></html>",
                article.title
            ).as_bytes(),
            "text/html"
        )
        .await
        .map_err(|err| Error::MinioCouldNotPutObject(err.to_string()))?;

    Ok(file_name)
}

pub async fn parse_comment_for_create(
    mut payload: Multipart,
    context: &Context,
    article_id: &Thing,
) -> Result<CommentForCreate, Error> {
    let mut comment = CommentForCreate::new();

    comment.user_id = context.user_id.clone();
    comment.article_id = article_id.clone();

    while let Some(field) = payload
        .next_field()
        .await
        .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?
    {
        if let Some(field_name) = field.name() {
            let name = field_name.to_string();
            let file_type = field.content_type();
            let file_name = field.file_name();

            if name == "content" {
                let data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?;
                comment.content = Some(parse_string_from_u8(&data)?);
            } else if name == "media" {
                comment.image = Some(Image::new());
                let mut image = comment
                    .image
                    .as_mut()
                    .expect("Unreachable, comment media should be contructed by now");
                if let Some(name) = file_name {
                    image.file_name = name.to_string();
                }
                if let Some(file_type) = file_type {
                    image.file_type = ImageType::from_str(file_type);
                    if !image.is_supported_image_type() {
                        return Err(Error::ServerUnsupportedMediaType(file_type.to_string()));
                    }
                }
                image.data = field
                    .bytes()
                    .await
                    .map_err(|err| Error::ServerCouldNotParseForm(err.to_string()))?
                    .to_vec();
            }
        }
    }

    Ok(comment)
}
