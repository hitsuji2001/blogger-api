use crate::database::{article::ARTICLE_FOLDER, Database};
use crate::errors::Error;
use crate::models::comment::{Comment, CommentForCreate};
use crate::server::context::Context;
use crate::{utils, utils::OpChanges};

use surrealdb::{opt::PatchOp, sql::Thing};

pub const COMMENT_TBL_NAME: &str = "comment";
pub const COMMENT_FOLDER: &str = "comments";

impl Database {
    pub async fn create_comment_table(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE TABLE comment SCHEMAFULL;
            DEFINE FIELD user_id                 ON TABLE comment TYPE record(user)      ASSERT $value != NONE;
            DEFINE FIELD article_id              ON TABLE comment TYPE record(article)   ASSERT $value != NONE;
            DEFINE FIELD reply                   ON TABLE comment TYPE array;
            DEFINE FIELD reply.*                 ON TABLE comment TYPE record(comment)   ASSERT $value != NONE;
            DEFINE FIELD content                 ON TABLE comment TYPE string;
            DEFINE FIELD deleted                 ON TABLE comment TYPE bool              ASSERT $value != NONE;
            DEFINE FIELD media_uri               ON TABLE comment TYPE string;
            DEFINE FIELD liked_by                ON TABLE comment TYPE array;
            DEFINE FIELD liked_by.*              ON TABLE comment TYPE record(user)      ASSERT $value != NONE;
            DEFINE FIELD created_at              ON TABLE comment TYPE datetime          ASSERT $value != NONE;
            DEFINE FIELD updated_at              ON TABLE comment TYPE datetime;
            DEFINE FIELD deleted_at              ON TABLE comment TYPE datetime;
            DEFINE INDEX liked_by_index          ON TABLE comment COLUMNS liked_by       UNIQUE;
        "#;

        self.client.query(sql).await.map_err(|err| {
            Error::DBCouldNotCreateTable(COMMENT_TBL_NAME.to_string(), err.to_string())
        })?;
        log::info!("Successfully create table: `{}`", COMMENT_TBL_NAME);

        Ok(())
    }

    pub async fn create_comment(&self, info: &mut CommentForCreate) -> Result<Thing, Error> {
        info.created_at = chrono::offset::Utc::now();

        let comment: Comment = self
            .client
            .create(COMMENT_TBL_NAME)
            .content(info)
            .await
            .map_err(|err| Error::DBCouldNotCreateRecord(err.to_string()))?;

        Ok(comment.id)
    }

    pub async fn create_reply(
        &self,
        context: &Context,
        (article_id, comment_id): &(Thing, Thing),
        info: &mut CommentForCreate,
    ) -> Result<Thing, Error> {
        info.created_at = chrono::offset::Utc::now();

        let comment: Comment = self
            .client
            .create(COMMENT_TBL_NAME)
            .content(&info)
            .await
            .map_err(|err| Error::DBCouldNotCreateRecord(err.to_string()))?;
        let id = comment.id.clone();

        if let Some(media) = &info.image {
            let uri = utils::multipart::upload_user_image_to_s3(
                format!(
                    "{}/{}/{}/{}",
                    context.user_id, ARTICLE_FOLDER, article_id, COMMENT_FOLDER
                )
                .as_str(),
                media,
            )
            .await?;

            self.update_comment_uri(&id, &uri).await?;
        }

        let changes: Vec<OpChanges> = self
            .client
            .update((comment_id.tb.clone(), comment_id.id.clone()))
            .patch(PatchOp::add("/reply", [id.clone()]))
            .await
            .map_err(|err| Error::DBCouldNotUpdateRecord(id.to_string(), err.to_string()))?;
        log::debug!(
            "Successfully create reply for comment: {}. Changes: {:?}",
            comment_id,
            changes
        );

        Ok(id)
    }

    pub async fn update_comment_uri(&self, comment: &Thing, uri: &String) -> Result<(), Error> {
        let changes: Vec<OpChanges> = self
            .client
            .update((comment.tb.clone(), comment.id.clone()))
            .patch(PatchOp::replace("/media_uri", uri))
            .await
            .map_err(|err| Error::DBCouldNotUpdateRecord(comment.to_string(), err.to_string()))?;

        log::debug!(
            "Successfully updated comment with id: `{}`, changes: {:?}",
            &comment,
            changes
        );
        Ok(())
    }

    pub async fn get_comment(&self, comment: &Thing) -> Result<Comment, Error> {
        let comment = self
            .client
            .select((comment.tb.clone(), comment.id.clone()))
            .await
            .map_err(|err| Error::DBCouldNotSelectRecord(comment.to_string(), err.to_string()))?;

        Ok(comment)
    }

    pub async fn delete_comment(&self, comment: &Thing) -> Result<(), Error> {
        let changes: Vec<OpChanges> = self
            .client
            .update((comment.tb.clone(), comment.id.clone()))
            .patch(PatchOp::replace("/deleted", true))
            .patch(PatchOp::replace("/deleted_at", chrono::offset::Utc::now()))
            .await
            .map_err(|err| Error::DBCouldNotDeleteRecord(comment.to_string(), err.to_string()))?;
        log::debug!(
            "Successfully marked comment with id: `{}` as deleted. Changes: {:?}",
            comment,
            changes
        );

        Ok(())
    }

    pub async fn get_reply_for_comment(&self, comment: &Thing) -> Result<Vec<Comment>, Error> {
        let get_reply_id_sql = format!("SELECT reply FROM comment WHERE id = {}", comment);
        let _ids: Vec<Thing> = self
            .client
            .query(get_reply_id_sql)
            .await
            .map_err(|err| Error::DBCouldNotSelectRecord(comment.to_string(), err.to_string()))?
            .take(0)
            .map_err(|err| Error::DBRecordEmpty(err.to_string()))?;

        let sql = format!(
            "SELECT * FROM comment WHERE (SELECT reply FROM comment WHERE id = {})",
            comment
        );
        let reply: Vec<Comment> = self
            .client
            .query(sql)
            .await
            .map_err(|err| Error::DBCouldNotSelectRecord(comment.to_string(), err.to_string()))?
            .take(0)
            .map_err(|err| Error::DBRecordEmpty(err.to_string()))?;

        Ok(reply)
    }

    pub async fn update_comment(
        &self,
        old_comment: &Comment,
        new_comment: &mut CommentForCreate,
    ) -> Result<(), Error> {
        let current_info = filter_empty_field(old_comment, new_comment).await?;
        let changes: Vec<OpChanges> = self
            .client
            .update((old_comment.id.tb.clone(), old_comment.id.id.clone()))
            .patch(PatchOp::replace("/content", current_info.content.clone()))
            .patch(PatchOp::replace(
                "/media_uri",
                current_info.media_uri.clone(),
            ))
            .patch(PatchOp::replace("/updated_at", current_info.updated_at))
            .patch(PatchOp::replace("/deleted", current_info.deleted))
            .await
            .map_err(|err| {
                Error::DBCouldNotUpdateRecord(old_comment.id.to_string(), err.to_string())
            })?;
        log::debug!(
            "Successfully create reply for comment: {}. Changes: {:?}",
            old_comment.id,
            changes
        );

        Ok(())
    }

    pub async fn get_comment_for_article(&self, article: &Thing) -> Result<Vec<Comment>, Error> {
        let sql = format!(
            "SELECT * FROM comment WHERE (SELECT comments FROM article WHERE id = {})",
            article
        );
        let comments: Vec<Comment> = self
            .client
            .query(sql)
            .await
            .map_err(|err| Error::DBCouldNotSelectRecord(article.to_string(), err.to_string()))?
            .take(0)
            .map_err(|err| Error::DBRecordEmpty(err.to_string()))?;

        Ok(comments)
    }
}

async fn filter_empty_field<'a>(
    old_comment: &Comment,
    new_comment: &'a mut CommentForCreate,
) -> Result<&'a mut CommentForCreate, Error> {
    if new_comment.content.is_none() && new_comment.image.is_none() {
        return Err(Error::ServerEmptyFormFromUser);
    }

    if let Some(content) = &new_comment.content {
        if content.is_empty() {
            new_comment.content = old_comment.content.clone();
        }
    } else if new_comment.content.is_none() {
        new_comment.content = old_comment.content.clone();
    }

    if let Some(media) = &new_comment.image {
        let uri = utils::multipart::upload_user_image_to_s3(
            format!(
                "{}/{}/{}/{}",
                old_comment.user_id, ARTICLE_FOLDER, old_comment.article_id, COMMENT_FOLDER
            )
            .as_str(),
            media,
        )
        .await?;

        new_comment.media_uri = Some(uri);
    } else {
        new_comment.media_uri = old_comment.media_uri.clone();
    }

    new_comment.updated_at = Some(chrono::offset::Utc::now());

    Ok(new_comment)
}
