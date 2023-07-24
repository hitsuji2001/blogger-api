use crate::database::Database;
use crate::errors::Error;
use crate::models::user::{User, UserForCreate};
use crate::server::context::Context;
use crate::{utils, utils::OpChanges};

use surrealdb::{opt::PatchOp, sql::Thing};

pub const USER_TBL_NAME: &str = "user";
const USER_PROFILE_FOLDER: &str = "user_profile_pictures";

impl Database {
    pub async fn create_user(&self, info: &mut UserForCreate) -> Result<String, Error> {
        info.created_at = chrono::offset::Utc::now();
        let user: User = self
            .client
            .create(USER_TBL_NAME)
            .content(info)
            .await
            .map_err(|err| Error::DBCouldNotCreateContent(err.to_string()))?;

        Ok(user.id.to_string())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let users: Vec<User> = self
            .client
            .select(USER_TBL_NAME)
            .await
            .map_err(|err| Error::DBCouldNotSelectAllUsers(err.to_string()))?;
        log::debug!("Successfully get all users");

        Ok(users)
    }

    pub async fn get_user_with_id(&self, id: &Thing) -> Result<User, Error> {
        let user: User = self
            .client
            .select((id.tb.clone(), id.id.clone()))
            .await
            .map_err(|err| Error::DBCouldNotSelectUser(id.to_string(), err.to_string()))?;
        log::debug!("Successfully get user with id: {}. user: {:?}", &id, &user);

        Ok(user)
    }

    pub async fn update_user_with_id(
        &self,
        id: &Thing,
        context: &Context,
        user: &UserForCreate,
    ) -> Result<(), Error> {
        let latest_config = self.get_user_with_id(id).await?;
        let current_info = filter_empty_field(user, &latest_config, &context).await?;

        let changes: Vec<OpChanges<String>> = self
            .client
            .update((id.tb.clone(), id.id.clone()))
            .patch(PatchOp::replace("/updated_at", &current_info.updated_at))
            .patch(PatchOp::replace("/first_name", &current_info.first_name))
            .patch(PatchOp::replace("/last_name", &current_info.last_name))
            .patch(PatchOp::replace(
                "/profile_pic_uri",
                &current_info.profile_pic_uri,
            ))
            .await
            .map_err(|err| Error::DBCouldNotUpdateUser(id.to_string(), err.to_string()))?;
        log::debug!(
            "Successfully updated user with id: `{}`, changes: {:?}",
            &id,
            changes
        );
        Ok(())
    }

    pub async fn delete_user_with_id(&self, id: &Thing) -> Result<User, Error> {
        let user: User = self
            .client
            .delete((id.tb.clone(), id.id.clone()))
            .await
            .map_err(|err| Error::DBCouldNotDeleteUser(id.to_string(), err.to_string()))?;

        log::debug!(
            "Successfully deleted user with: id `{}`. user: {:?}",
            &id,
            &user
        );

        Ok(user)
    }

    pub async fn get_user_with_email(&self, email: &String) -> Result<User, Error> {
        let sql = format!("SELECT * FROM {} WHERE email == '{}'", USER_TBL_NAME, email);
        let users: Vec<User> = self
            .client
            .query(sql)
            .await
            .map_err(|err| Error::DBCouldNotSelectUser(email.to_string(), err.to_string()))?
            .take(0)
            .map_err(|err| Error::DBCouldNotSelectUser(email.to_string(), err.to_string()))?;

        if users.len() == 0 {
            return Err(Error::DBCouldNotSelectUser(
                email.to_string(),
                "There's no user with such email".to_string(),
            ));
        } else if users.len() > 1 {
            return Err(Error::DBDuplicateUserEmail);
        }

        Ok(users[0].clone())
    }

    // BUG: Find a way to connect articles with user
    pub async fn create_user_table(&self) -> Result<(), Error> {
        let sql = r#"
            DEFINE TABLE user SCHEMAFULL;
            DEFINE FIELD username           ON TABLE user TYPE string   ASSERT $value != NONE;
            DEFINE FIELD first_name         ON TABLE user TYPE string   ASSERT $value != NONE;
            DEFINE FIELD last_name          ON TABLE user TYPE string   ASSERT $value != NONE;
            DEFINE FIELD email              ON TABLE user TYPE string   ASSERT $value != NONE AND is::email($value);
            DEFINE FIELD profile_pic_uri    ON TABLE user TYPE string;
            DEFINE FIELD created_at         ON TABLE user TYPE datetime ASSERT $value != NONE;
            DEFINE FIELD updated_at         ON TABLE user TYPE datetime;
            DEFINE FIELD is_admin           ON TABLE user TYPE bool;
            DEFINE FIELD articles           ON TABLE user TYPE array;
            DEFINE FIELD articles.*         ON TABLE user TYPE record(article) ASSERT $value != NONE;
            DEFINE INDEX article_index      ON TABLE user COLUMNS articles.* UNIQUE;
            DEFINE INDEX username_index     ON TABLE user COLUMNS username UNIQUE;
            DEFINE INDEX user_email_index   ON TABLE user COLUMNS email UNIQUE;
        "#;

        self.client
            .query(sql)
            .await
            .map_err(|err| Error::DBCouldNotCreateTable(String::from("user"), err.to_string()))?;
        log::info!("Create `user` table successfully");

        Ok(())
    }
}
async fn filter_empty_field(
    current: &UserForCreate,
    latest: &User,
    context: &Context,
) -> Result<UserForCreate, Error> {
    if current.first_name == "" && current.last_name == "" && current.avatar == None {
        return Err(Error::ServerEmptyFormFromUser);
    }

    let mut result = UserForCreate {
        first_name: latest.first_name.clone(),
        last_name: latest.last_name.clone(),
        username: Default::default(),
        email: Default::default(),
        is_admin: latest.is_admin,
        avatar: Default::default(),
        profile_pic_uri: latest.profile_pic_uri.clone(),
        created_at: latest.created_at,
        updated_at: latest.updated_at,
    };

    if let Some(_) = &current.avatar {
        result.profile_pic_uri = Some(
            utils::multipart::upload_user_image_to_s3(
                current,
                format!("{}/{}", context.user_id, USER_PROFILE_FOLDER).as_str(),
            )
            .await?,
        );
    };

    if current.first_name != "" {
        result.first_name = current.first_name.clone();
    }
    if current.last_name != "" {
        result.last_name = current.last_name.clone();
    }

    result.updated_at = Some(chrono::offset::Utc::now());

    Ok(result)
}
