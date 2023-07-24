use crate::database::Database;
use crate::errors::Error;
use crate::models::article::{Article, ArticleForCreate};
use crate::utils;

const ARTICLE_FOLDER: &str = "articles";
const ARTICLE_TBL_NAME: &str = "article";

impl Database {
    pub async fn create_article_table(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE TABLE article SCHEMAFULL;
            DEFINE FIELD user_id              ON TABLE article TYPE record(user)   ASSERT $value != NONE;
            DEFINE FIELD title                ON TABLE article TYPE string         ASSERT $value != NONE;
            DEFINE FIELD public               ON TABLE article TYPE bool           ASSERT $value != NONE;
            DEFINE FIELD tags                 ON TABLE article TYPE array;
            DEFINE FIELD tags.*               ON TABLE article TYPE string;
            DEFINE FIELD article_uri          ON TABLE article TYPE string;
            DEFINE FIELD images_uri_list      ON TABLE article TYPE array;
            DEFINE FIELD images_uri_list.*    ON TABLE article TYPE string;
            DEFINE FIELD created_at           ON TABLE article TYPE datetime       ASSERT $value != NONE;
            DEFINE FIELD updated_at           ON TABLE article TYPE datetime;
        "#;

        self.client.query(sql).await.map_err(|err| {
            Error::DBCouldNotCreateTable(String::from("article"), err.to_string())
        })?;
        log::info!("Create `article` table successfully");

        Ok(())
    }

    pub async fn create_article(&self, info: &mut ArticleForCreate) -> Result<String, Error> {
        info.created_at = chrono::offset::Utc::now();
        info.article_uri = utils::multipart::upload_html_to_s3(
            info,
            format!("{}/{}", info.user_id, ARTICLE_FOLDER).as_str(),
        )
        .await?;

        let article: Article = self
            .client
            .create(ARTICLE_TBL_NAME)
            .content(info)
            .await
            .map_err(|err| Error::DBCouldNotCreateContent(err.to_string()))?;

        Ok(article.id.to_string())
    }
}
