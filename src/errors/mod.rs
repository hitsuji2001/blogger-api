pub mod database;
pub mod env;
pub mod server;

use crate::errors::{database::DBError, env::EnvError, server::ServerError};

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
    ParseEnvFailedNoSuchFile,
    ParseEnvFailedNoSuchKey { key: String },
    ServerNoSuchIP,
    ServerCouldNotStart,
    ServerInvalidRegex,
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
        }
    }
}

impl From<ServerError> for Error {
    fn from(err: ServerError) -> Self {
        match err {
            ServerError::NoSuchIP => Error::ServerNoSuchIP,
            ServerError::InvalidRegex => Error::ServerInvalidRegex,
            ServerError::CouldNotStartServer => Error::ServerCouldNotStart,
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
        }
    }
}
