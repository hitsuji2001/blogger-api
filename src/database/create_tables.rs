use crate::errors::Error;
use surrealdb::{engine::remote::ws::Client, Surreal};

// TODO: Find a way to migrate data
// NOTE: Remember that table field here must match field name in Struct decalration

pub async fn article(db: &Surreal<Client>) -> Result<(), Error> {
    let sql = r#"
        DEFINE TABLE article SCHEMAFULL;
        DEFINE FIELD user_id              ON TABLE article TYPE string   ASSERT $value != NONE;
        DEFINE FIELD title                ON TABLE article TYPE string   ASSERT $value != NONE;
        DEFINE FIELD public               ON TABLE article TYPE bool     ASSERT $value != NONE;
        DEFINE FIELD content              ON TABLE article TYPE string;
        DEFINE FIELD tags                 ON TABLE article TYPE array;
        DEFINE FIELD tags.*               ON TABLE article TYPE string;
        DEFINE FIELD article_uri          ON TABLE article TYPE string;
        DEFINE FIELD images_uri_list      ON TABLE article TYPE array;
        DEFINE FIELD images_uri_list.*    ON TABLE article TYPE string;
        DEFINE FIELD created_at           ON TABLE article TYPE datetime ASSERT $value != NONE;
        DEFINE FIELD updated_at           ON TABLE article TYPE datetime;
        DEFINE INDEX user_id_index        ON TABLE article COLUMNS user_id UNIQUE;
        "#;

    db.query(sql)
        .await
        .map_err(|err| Error::DBCouldNotCreateTable(String::from("article"), err.to_string()))?;
    log::info!("Create `article` table successfully");

    Ok(())
}

pub async fn user(db: &Surreal<Client>) -> Result<(), Error> {
    let sql = r#"
        DEFINE TABLE user SCHEMAFULL;
        DEFINE FIELD username           ON TABLE user TYPE string   ASSERT $value != NONE;
        DEFINE FIELD first_name         ON TABLE user TYPE string   ASSERT $value != NONE;
        DEFINE FIELD last_name          ON TABLE user TYPE string   ASSERT $value != NONE;
        DEFINE FIELD email              ON TABLE user TYPE string   ASSERT $value != NONE AND is::email($value);
        DEFINE FIELD profile_pic_uri    ON TABLE user TYPE string;
        DEFINE FIELD created_at         ON TABLE user TYPE datetime ASSERT $value != NONE;
        DEFINE FIELD updated_at         ON TABLE user TYPE datetime;
        DEFINE FIELD articles           ON TABLE user TYPE array;
        DEFINE FIELD articles.*         ON TABLE user TYPE string ASSERT $value != NONE;
        DEFINE INDEX username_index     ON TABLE user COLUMNS username UNIQUE;
        DEFINE INDEX user_email_index   ON TABLE user COLUMNS email UNIQUE;
        "#;

    db.query(sql)
        .await
        .map_err(|err| Error::DBCouldNotCreateTable(String::from("user"), err.to_string()))?;
    log::info!("Create `user` table successfully");

    Ok(())
}
