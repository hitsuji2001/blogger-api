use crate::errors::{database::DBError, Error};
use surrealdb::{engine::remote::ws::Client, Surreal};

// #[allow(dead_code)]
// async fn blogpost(db: &Surreal<Client>) -> Result<(), Error> {
//
// }

#[allow(dead_code)]
pub async fn user(db: &Surreal<Client>) -> Result<(), Error> {
    let sql = r#"
        DEFINE TABLE user SCHEMAFULL;
        DEFINE FIELD first_name       ON TABLE user TYPE string   ASSERT $value != NONE;
        DEFINE FIELD last_name        ON TABLE user TYPE string   ASSERT $value != NONE;
        DEFINE FIELD email            ON TABLE user TYPE string   ASSERT $value != NONE AND is::email($value);
        DEFINE FIELD profile_pic_uri  ON TABLE user TYPE string;
        DEFINE FIELD created_at       ON TABLE user TYPE datetime ASSERT $value != NONE;
        DEFINE FIELD updated_at       ON TABLE user TYPE datetime;
        "#;
    // DEFINE INDEX user_email_index ON TABLE user COLUMNS email UNIQUE;

    db.query(sql).await.map_err(|err| {
        log::error!("Could not create table user.\n    --> Cause: {}", err);
        DBError::TableCreateFailed
    })?;

    Ok(())
}
