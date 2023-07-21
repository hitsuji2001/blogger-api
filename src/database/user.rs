use crate::database::Database;
use crate::errors::Error;
use crate::models::user::{User, UserForCreate, USER_TBL_NAME};
use crate::server::context::Context;
use crate::utils;

use serde::Deserialize;
use surrealdb::opt::PatchOp;

const USER_PROFILE_FOLDER: &str = "user_profile_pictures";

impl Database {
    pub async fn create_user(&self, info: &mut UserForCreate) -> Result<String, Error> {
        info.created_at = chrono::offset::Utc::now();
        let user: User = self
            .client
            .create(USER_TBL_NAME)
            .content(info)
            .await
            .map_err(|err| Error::DBCouldNotCreateUser(err.to_string()))?;

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

    pub async fn get_user_with_id(&self, id: &String) -> Result<User, Error> {
        let user: User = self
            .client
            .select((USER_TBL_NAME, id))
            .await
            .map_err(|err| Error::DBCouldNotSelectUser(id.clone(), err.to_string()))?;
        log::debug!("Successfully get user with id: {}. user: {:?}", &id, &user);

        Ok(user)
    }

    pub async fn update_user_with_id(
        &self,
        id: &String,
        context: &Context,
        user: &UserForCreate,
    ) -> Result<(), Error> {
        let mut file_path = String::default();
        if let Some(user_avatar) = &user.avatar_as_bytes {
            file_path = utils::multipart::upload_to_s3(
                format!("{}/{}", context.user_id, USER_PROFILE_FOLDER).as_str(),
                &user_avatar,
            )
            .await?;
        };

        let changes: Vec<OpChanges> = self
            .client
            .update((USER_TBL_NAME, id))
            .patch(PatchOp::replace("/updated_at", chrono::offset::Utc::now()))
            .patch(PatchOp::replace("/first_name", &user.first_name))
            .patch(PatchOp::replace("/last_name", &user.last_name))
            .patch(PatchOp::replace("/profile_pic_uri", &file_path))
            .await
            .map_err(|err| Error::DBCouldNotUpdateUser(id.clone(), err.to_string()))?;
        log::debug!(
            "Successfully updated user with id: `{}`, changes: {:?}",
            &id,
            changes
        );
        Ok(())
    }

    pub async fn delete_user_with_id(&self, id: &String) -> Result<User, Error> {
        let user: User = self
            .client
            .delete((USER_TBL_NAME, id))
            .await
            .map_err(|err| Error::DBCouldNotDeleteUser(id.clone(), err.to_string()))?;

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
                "There's is no user with such email".to_string(),
            ));
        } else if users.len() > 1 {
            return Err(Error::DBDuplicateUserEmail);
        }

        Ok(users[0].clone())
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OpChanges {
    op: String,
    path: String,
    value: String,
}
