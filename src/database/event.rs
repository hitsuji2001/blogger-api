use crate::database::Database;
use crate::errors::Error;

impl Database {
    pub async fn create_events(&self) -> Result<(), Error> {
        self.on_create_article().await?;
        self.on_create_comment().await?;

        Ok(())
    }

    async fn on_create_article(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE EVENT on_create_article ON TABLE article WHEN $event = "CREATE" THEN {
                LET $from = $after.user_id;
                LET $to   = $after.id;
                
                RELATE $from->wrote->$to;
                UPDATE user SET articles += $after.id WHERE id = $after.user_id;
            };
        "#;
        self.client
            .query(sql)
            .await
            .map_err(|err| Error::DBCouldNotCreateEvent(err.to_string()))?;
        log::info!("Successfully create events: `on_create_article`");

        Ok(())
    }

    async fn on_create_comment(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE EVENT on_create_comment ON TABLE comment WHEN $event = "CREATE" THEN {
                LET $from = $after.user_id;
                LET $to   = $after.article_id;
                
                RELATE $from->comment_on->$to;
                UPDATE article SET comments += $after.id WHERE id = $after.article_id;
            };
        "#;
        self.client
            .query(sql)
            .await
            .map_err(|err| Error::DBCouldNotCreateEvent(err.to_string()))?;
        log::info!("Successfully create events: `on_create_comment`");

        Ok(())
    }
}
