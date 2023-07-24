use crate::database::Database;
use crate::errors::Error;

impl Database {
    pub async fn create_events(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE EVENT create_article ON TABLE article WHEN $event = "CREATE" THEN {
                LET $from = $after.user_id;
                LET $to   = $after.id;
                
                RELATE $from->wrote->$to;
                UPDATE user SET articles += $after.id WHERE id = $after.user_id;
            };
        "#;
        self.client
            .query(sql)
            .await
            .map_err(|err| Error::DBCouldNotCreateTable(String::from("user"), err.to_string()))?;
        log::info!("Successfully create events");

        Ok(())
    }
}
