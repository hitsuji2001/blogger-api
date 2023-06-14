use crate::errors::{database::DBError, Error};
use surrealdb::{engine::remote::ws::Client, Surreal};

// async fn create_blogpost(db: &Surreal<Client>) -> Result<(), Error> {
//
// }

#[allow(dead_code)]
pub async fn create_user(db: &Surreal<Client>) -> Result<(), Error> {
    let sql = r#"
        DEFINE TABLE user SCHEMAFULL DROP;
        DEFINE FIELD first_name ON TABLE user TYPE string
            ASSERT $value != NONE;
        DEFINE FIELD last_name ON TABLE user TYPE string
            ASSERT $value != NONE;
        DEFINE FIELD email ON TABLE user TYPE string
            ASSERT $value != NONE AND is::email($value);
        DEFINE FIELD created_at ON TABLE user TYPE datetime
            VALUE time::now();
        DEFINE FIELD updated_at ON TABLE user TYPE datetime
            VALUE NULL;
        -- DEFINE INDEX user_email_index ON TABLE user COLUMNS email UNIQUE;
        "#;

    db.query(sql).await.map_err(|err| {
        log::error!("Could not create table user.\n    --> Cause: {}", err);
        DBError::TableCreateFailed
    })?;

    Ok(())
}
