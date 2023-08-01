use crate::database::{article::ARTICLE_TBL_NAME, comment::COMMENT_TBL_NAME, Database};
use crate::errors::Error;
use crate::models::{article::Article, comment::Comment};
use crate::server::context::Context;
use crate::utils::OpChanges;

use surrealdb::{opt::PatchOp, sql::Thing};

const LIKE_TBL_NAME: &str = "likes";

impl Database {
    pub async fn create_like_table(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE INDEX unique_relationships 
            ON TABLE likes 
            COLUMNS in, out UNIQUE;
        "#;

        self.client.query(sql).await.map_err(|err| {
            Error::DBCouldNotCreateTable(LIKE_TBL_NAME.to_string(), err.to_string())
        })?;
        log::info!("Successfully create table: `{}`", LIKE_TBL_NAME);

        Ok(())
    }

    pub async fn like_comment_or_article(
        &self,
        context: &Context,
        id: &Thing,
    ) -> Result<Thing, Error> {
        let sql = format!("RELATE {}->likes->{}", context.user_id, id);
        let mut response = self.client.query(sql).await.map_err(|err| {
            Error::DBCouldNotRelateRecord(
                context.user_id.to_string(),
                id.to_string(),
                err.to_string(),
            )
        })?;
        let like: Option<Thing> = response
            .take("id")
            .map_err(|err| Error::DBRecordAlreadyExist(id.to_string(), err.to_string()))?;
        let changes: Vec<OpChanges> = self
            .client
            .update((id.tb.clone(), id.id.clone()))
            .patch(PatchOp::add("/liked_by", [context.user_id.clone()]))
            .await
            .map_err(|err| Error::DBRecordAlreadyExist(id.to_string(), err.to_string()))?;
        log::debug!(
            "Successfully add like to: `{}`, changes: {:?}",
            &id,
            changes
        );

        if let Some(like) = like {
            Ok(like)
        } else {
            Err(Error::DBCouldNotRelateRecord(
                context.user_id.to_string(),
                id.to_string(),
                "".to_string(),
            ))
        }
    }

    pub async fn unlike_comment_or_article(
        &self,
        context: &Context,
        id: &Thing,
    ) -> Result<(), Error> {
        let sql = format!(
            "DELETE likes WHERE in = {} AND out = {}",
            context.user_id, id
        );
        self.client.query(sql).await.map_err(|err| {
            Error::DBCouldNotDeleteRelateRecord(
                context.user_id.to_string(),
                id.to_string(),
                err.to_string(),
            )
        })?;
        let mut position: Option<usize> = Default::default();

        if id.tb == ARTICLE_TBL_NAME {
            let article: Article = self
                .client
                .select((id.tb.clone(), id.id.clone()))
                .await
                .map_err(|err| Error::DBCouldNotSelectRecord(id.to_string(), err.to_string()))?;

            if let Some(user_ids) = article.liked_by {
                position = user_ids
                    .iter()
                    .position(|element| element == &context.user_id);
            } else {
                return Err(Error::DBRecordDidNotExist(context.user_id.to_string()));
            }
        } else if id.tb == COMMENT_TBL_NAME {
            let comment: Comment = self
                .client
                .select((id.tb.clone(), id.id.clone()))
                .await
                .map_err(|err| Error::DBCouldNotSelectRecord(id.to_string(), err.to_string()))?;

            if let Some(user_ids) = comment.liked_by {
                position = user_ids
                    .iter()
                    .position(|element| element == &context.user_id);
            } else {
                return Err(Error::DBRecordDidNotExist(context.user_id.to_string()));
            }
        }

        if let Some(position) = position {
            let changes: Vec<OpChanges> = self
                .client
                .update((id.tb.clone(), id.id.clone()))
                .patch(PatchOp::remove(format!("/liked_by/{}", position).as_str()))
                .await
                .map_err(|err| Error::DBCouldNotSelectRecord(id.to_string(), err.to_string()))?;
            log::debug!(
                "Successfully remove like to: `{}`, changes: {:?}",
                &id,
                changes
            );

            Ok(())
        } else {
            Err(Error::DBRecordDidNotExist(context.user_id.to_string()))
        }
    }
}
