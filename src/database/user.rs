use crate::database::Database;
use crate::errors::Error;
use crate::models::user::{User, UserForCreate};
use crate::server::context::Context;
use crate::{utils, utils::OpChanges};

use surrealdb::{opt::PatchOp, sql::Thing};

pub const USER_TBL_NAME: &str = "user";
const USER_PROFILE_FOLDER: &str = "user_profile_pictures";

impl Database {
    pub async fn create_user_table(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE TABLE user SCHEMAFULL;
            DEFINE FIELD username           ON TABLE user TYPE string          ASSERT $value != NONE;
            DEFINE FIELD first_name         ON TABLE user TYPE string          ASSERT $value != NONE;
            DEFINE FIELD last_name          ON TABLE user TYPE string          ASSERT $value != NONE;
            DEFINE FIELD email              ON TABLE user TYPE string          ASSERT $value != NONE AND is::email($value);
            DEFINE FIELD profile_pic_uri    ON TABLE user TYPE string;
            DEFINE FIELD created_at         ON TABLE user TYPE datetime        ASSERT $value != NONE;
            DEFINE FIELD updated_at         ON TABLE user TYPE datetime;       
            DEFINE FIELD deleted_at         ON TABLE user TYPE datetime;       
            DEFINE FIELD is_admin           ON TABLE user TYPE bool            ASSERT $value != NONE;
            DEFINE FIELD deleted            ON TABLE user TYPE bool            ASSERT $value != NONE;
            DEFINE FIELD articles           ON TABLE user TYPE array;
            DEFINE FIELD articles.*         ON TABLE user TYPE record(article) ASSERT $value != NONE;
            DEFINE INDEX article_index      ON TABLE user COLUMNS articles.*   UNIQUE;
            DEFINE INDEX username_index     ON TABLE user COLUMNS username     UNIQUE;
            DEFINE INDEX user_email_index   ON TABLE user COLUMNS email        UNIQUE;
        "#;

        self.client.query(sql).await.map_err(|err| {
            Error::DBCouldNotCreateTable(USER_TBL_NAME.to_string(), err.to_string())
        })?;
        log::info!("Successfully create table: `{}`", USER_TBL_NAME);

        Ok(())
    }

    pub async fn create_user(&self, info: &mut UserForCreate) -> Result<String, Error> {
        info.created_at = chrono::offset::Utc::now();
        let user: User = self
            .client
            .create(USER_TBL_NAME)
            .content(info)
            .await
            .map_err(|err| Error::DBCouldNotCreateRecord(err.to_string()))?;

        Ok(user.id.to_string())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let users: Vec<User> = self
            .client
            .select(USER_TBL_NAME)
            .await
            .map_err(|err| Error::DBCouldNotSelectAllRecords(err.to_string()))?;
        log::debug!("Successfully get all users");

        Ok(users)
    }

    pub async fn get_user_with_id(&self, id: &Thing) -> Result<User, Error> {
        let user: User = self
            .client
            .select((id.tb.clone(), id.id.clone()))
            .await
            .map_err(|err| Error::DBCouldNotSelectRecord(id.to_string(), err.to_string()))?;
        log::debug!("Successfully get user with id: {}. user: {:?}", &id, &user);

        Ok(user)
    }

    pub async fn update_user_with_id(
        &self,
        id: &Thing,
        context: &Context,
        user: &UserForCreate,
    ) -> Result<(), Error> {
        let old_user = self.get_user_with_id(id).await?;
        let new_user = filter_empty_field(user, &old_user, context).await?;

        let changes: Vec<OpChanges> = self
            .client
            .update((id.tb.clone(), id.id.clone()))
            .patch(PatchOp::replace("/updated_at", new_user.updated_at))
            .patch(PatchOp::replace("/first_name", &new_user.first_name))
            .patch(PatchOp::replace("/last_name", &new_user.last_name))
            .patch(PatchOp::replace(
                "/profile_pic_uri",
                &new_user.profile_pic_uri,
            ))
            .await
            .map_err(|err| Error::DBCouldNotUpdateRecord(id.to_string(), err.to_string()))?;
        log::debug!(
            "Successfully updated user with id: `{}`, changes: {:?}",
            &id,
            changes
        );
        Ok(())
    }

    pub async fn delete_user_with_id(&self, user: &Thing) -> Result<(), Error> {
        let changes: Vec<OpChanges> = self
            .client
            .update((user.tb.clone(), user.id.clone()))
            .patch(PatchOp::replace("/deleted", true))
            .patch(PatchOp::replace("/deleted_at", chrono::offset::Utc::now()))
            .await
            .map_err(|err| Error::DBCouldNotDeleteRecord(user.to_string(), err.to_string()))?;
        log::debug!(
            "Successfully marked user with id: `{}` as deleted. Changes: {:?}",
            user,
            changes
        );

        Ok(())
    }

    pub async fn get_user_with_email(&self, email: &String) -> Result<User, Error> {
        let sql = format!("SELECT * FROM {} WHERE email == '{}'", USER_TBL_NAME, email);
        let users: Vec<User> = self
            .client
            .query(sql)
            .await
            .map_err(|err| Error::DBCouldNotSelectRecord(email.to_string(), err.to_string()))?
            .take(0)
            .map_err(|err| Error::DBRecordEmpty(err.to_string()))?;

        if users.is_empty() {
            return Err(Error::DBCouldNotSelectRecord(
                email.to_string(),
                "There's no user with such email".to_string(),
            ));
        } else if users.len() > 1 {
            return Err(Error::DBDuplicateUserEmail);
        }

        Ok(users[0].clone())
    }
}

async fn filter_empty_field(
    new_user: &UserForCreate,
    old_user: &User,
    context: &Context,
) -> Result<UserForCreate, Error> {
    if new_user.first_name.is_empty() && new_user.last_name.is_empty() && new_user.avatar.is_none()
    {
        return Err(Error::ServerEmptyFormFromUser);
    }

    let mut result = UserForCreate {
        first_name: old_user.first_name.clone(),
        last_name: old_user.last_name.clone(),
        username: Default::default(),
        email: Default::default(),
        is_admin: old_user.is_admin,
        deleted: old_user.deleted,
        avatar: Default::default(),
        profile_pic_uri: old_user.profile_pic_uri.clone(),
        created_at: old_user.created_at,
        updated_at: old_user.updated_at,
        deleted_at: old_user.deleted_at,
    };

    if let Some(avatar) = &new_user.avatar {
        result.profile_pic_uri = Some(
            utils::multipart::upload_user_image_to_s3(
                format!("{}/{}", context.user_id, USER_PROFILE_FOLDER).as_str(),
                avatar,
            )
            .await?,
        );
    };

    if !new_user.first_name.is_empty() {
        result.first_name = new_user.first_name.clone();
    }
    if !new_user.last_name.is_empty() {
        result.last_name = new_user.last_name.clone();
    }

    result.updated_at = Some(chrono::offset::Utc::now());

    Ok(result)
}
