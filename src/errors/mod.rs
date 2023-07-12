pub mod database;
pub mod env;
pub mod s3;
pub mod server;

use crate::errors::{database::DBError, env::EnvError, s3::S3Error, server::ServerError};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    DBCouldNotOpenWS,
    DBAuthFailed,
    DBCouldNotCreateTable,
    DBCouldNotCreateUser,
    DBNamespaceNotFound,
    DBCouldNotSelectUser,
    DBCouldNotDeleteUser,
    DBCouldNotUpdateUser,
    ParseEnvFailedNoSuchFile,
    ParseEnvFailedNoSuchKey { key: String },
    ParseEnvFailedWrongFormat,
    ServerNoSuchIP,
    ServerCouldNotStart,
    ServerInvalidRegex,
    ServerCouldNotParseUserForm,
    MinioCouldNotInitBucket,
    MinioCouldNotPutObject,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        log::info!("Handler::Error::IntoResponse");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        return response;
    }
}

impl From<EnvError> for Error {
    fn from(err: EnvError) -> Self {
        match err {
            EnvError::NoSuchFile => Error::ParseEnvFailedNoSuchFile,
            EnvError::NoSuchKey { key } => Error::ParseEnvFailedNoSuchKey { key },
            EnvError::WrongFormat => Error::ParseEnvFailedWrongFormat,
        }
    }
}

impl From<ServerError> for Error {
    fn from(err: ServerError) -> Self {
        match err {
            ServerError::NoSuchIP => Error::ServerNoSuchIP,
            ServerError::InvalidRegex => Error::ServerInvalidRegex,
            ServerError::CouldNotStartServer => Error::ServerCouldNotStart,
            ServerError::CouldNotParseUserForm => Error::ServerCouldNotParseUserForm,
        }
    }
}

impl From<DBError> for Error {
    fn from(err: DBError) -> Self {
        match err {
            DBError::CouldNotOpenWebSocket => Error::DBCouldNotOpenWS,
            DBError::AuthFailed => Error::DBAuthFailed,
            DBError::CouldNotConnectToNameSpace => Error::DBNamespaceNotFound,
            DBError::TableCreateFailed => Error::DBCouldNotCreateTable,
            DBError::UserCreateFailed => Error::DBCouldNotCreateUser,
            DBError::UserSelectFailed => Error::DBCouldNotSelectUser,
            DBError::UserDeleteFailed => Error::DBCouldNotDeleteUser,
            DBError::UserUpdateFailed => Error::DBCouldNotUpdateUser,
        }
    }
}

impl From<S3Error> for Error {
    fn from(err: S3Error) -> Self {
        match err {
            S3Error::CouldNotInitBucket => Error::MinioCouldNotInitBucket,
            S3Error::CouldNotPutObject => Error::MinioCouldNotPutObject,
        }
    }
}
