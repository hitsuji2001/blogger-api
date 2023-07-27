use crate::database::Database;
use crate::errors::Error;

pub const COMMENT_TBL_NAME: &str = "comment";

impl Database {
    pub async fn create_comment_table(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE TABLE comment SCHEMAFULL;
            DEFINE FIELD user_id                 ON TABLE comment TYPE record(user)      ASSERT $value != NONE;
            DEFINE FIELD article_id              ON TABLE comment TYPE record(article)   ASSERT $value != NONE;
            DEFINE FIELD reply                   ON TABLE comment TYPE record(comment);
            DEFINE FIELD text                    ON TABLE comment TYPE string;
            DEFINE FIELD media_uri               ON TABLE comment TYPE string;
            DEFINE FIELD liked_by                ON TABLE comment TYPE array;
            DEFINE FIELD liked_by.*              ON TABLE comment TYPE record(user)      ASSERT $value != NONE;
            DEFINE FIELD created_at              ON TABLE comment TYPE datetime          ASSERT $value != NONE;
            DEFINE FIELD updated_at              ON TABLE comment TYPE datetime;
        "#;

        self.client.query(sql).await.map_err(|err| {
            Error::DBCouldNotCreateTable(COMMENT_TBL_NAME.to_string(), err.to_string())
        })?;
        log::info!("Successfully create table: `{}`", COMMENT_TBL_NAME);

        Ok(())
    }
}
