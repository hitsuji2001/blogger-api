use crate::database::Database;
use crate::errors::Error;
use crate::models::article::{Article, ArticleForCreate};
use crate::server::context::Context;
use crate::utils::OpChanges;

use surrealdb::{opt::PatchOp, sql::Thing};

pub const ARTICLE_FOLDER: &str = "articles";
pub const ARTICLE_TBL_NAME: &str = "article";

impl Database {
    pub async fn create_article_table(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE TABLE article SCHEMAFULL;
            DEFINE FIELD user_id              ON TABLE article TYPE record(user)    ASSERT $value != NONE;
            DEFINE FIELD title                ON TABLE article TYPE string          ASSERT $value != NONE;
            DEFINE FIELD public               ON TABLE article TYPE bool            ASSERT $value != NONE;
            DEFINE FIELD tags                 ON TABLE article TYPE array;
            DEFINE FIELD tags.*               ON TABLE article TYPE string;
            DEFINE FIELD article_uri          ON TABLE article TYPE string;
            DEFINE FIELD comments             ON TABLE article TYPE array;
            DEFINE FIELD comments.*           ON TABLE article TYPE record(comment) ASSERT $value != NONE;
            DEFINE FIELD liked_by             ON TABLE article TYPE array;
            DEFINE FIELD liked_by.*           ON TABLE article TYPE record(user)    ASSERT $value != NONE;
            DEFINE FIELD images_uri_list      ON TABLE article TYPE array;
            DEFINE FIELD images_uri_list.*    ON TABLE article TYPE string;
            DEFINE FIELD created_at           ON TABLE article TYPE datetime        ASSERT $value != NONE;
            DEFINE FIELD updated_at           ON TABLE article TYPE datetime;
        "#;

        self.client.query(sql).await.map_err(|err| {
            Error::DBCouldNotCreateTable(ARTICLE_TBL_NAME.to_string(), err.to_string())
        })?;
        log::info!("Successfully create table: `{}`", ARTICLE_TBL_NAME);

        Ok(())
    }

    pub async fn create_article(&self, info: &mut ArticleForCreate) -> Result<Thing, Error> {
        info.created_at = chrono::offset::Utc::now();
        info.article_uri = String::from("");

        let article: Article = self
            .client
            .create(ARTICLE_TBL_NAME)
            .content(info)
            .await
            .map_err(|err| Error::DBCouldNotCreateRecord(err.to_string()))?;

        Ok(article.id)
    }

    pub async fn list_articles_for_user(&self, context: &Context) -> Result<Vec<Article>, Error> {
        let sql = format!(
            "SELECT * FROM article WHERE (SELECT ->wrote->article FROM user WHERE id = {})",
            context.user_id
        );

        let articles: Vec<Article> = self
            .client
            .query(sql)
            .await
            .map_err(|err| {
                Error::DBCouldNotSelectRecord(context.user_id.to_string(), err.to_string())
            })?
            .take(0)
            .map_err(|err| Error::DBRecordEmpty(err.to_string()))?;

        Ok(articles)
    }

    pub async fn get_article_with_id(&self, id: &Thing) -> Result<Article, Error> {
        let article: Article = self
            .client
            .select((id.tb.clone(), id.id.clone()))
            .await
            .map_err(|err| Error::DBCouldNotSelectRecord(id.to_string(), err.to_string()))?;

        log::debug!(
            "Successfully get article with id: {}. article: {:?}",
            &id,
            &article
        );

        Ok(article)
    }

    pub async fn delete_article_with_id(&self, id: &Thing) -> Result<Article, Error> {
        let article: Article = self
            .client
            .delete((id.tb.clone(), id.id.clone()))
            .await
            .map_err(|err| Error::DBCouldNotDeleteRecord(id.to_string(), err.to_string()))?;

        log::debug!(
            "Successfully delete article with id: {}. article: {:?}",
            &id,
            &article
        );

        Ok(article)
    }

    pub async fn update_article_uri(&self, article_id: &Thing, uri: &str) -> Result<(), Error> {
        let changes: Vec<OpChanges> = self
            .client
            .update((article_id.tb.clone(), article_id.id.clone()))
            .patch(PatchOp::replace("/article_uri", uri))
            .await
            .map_err(|err| {
                Error::DBCouldNotUpdateRecord(article_id.to_string(), err.to_string())
            })?;

        log::debug!(
            "Successfully updated article with id: `{}`, changes: {:?}",
            &article_id,
            changes
        );
        Ok(())
    }
}
