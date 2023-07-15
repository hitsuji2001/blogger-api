use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// TODO: Maybe find a better way to handle error
#[derive(Debug)]
pub enum Error {
    // The last String will always be error message when map_err
    DBCouldNotOpenWebSocket(String, String),
    DBAuthenticationFailed(String),
    DBCouldNotCreateTable(String, String),
    DBCouldNotConnectToNamespace(String, String),
    DBCouldNotCreateUser(String),
    DBCouldNotSelectAllUsers(String),
    DBCouldNotSelectUser(String, String),
    DBCouldNotDeleteUser(String, String),
    DBCouldNotUpdateUser(String, String),

    ParseEnvFailedNoSuchFile(String, String),
    ParseEnvFailedNoSuchKey(String),
    ParseEnvFailedWrongFormat(String),

    ServerNoSuchIP(String, String),
    ServerCouldNotStart(String),
    ServerInvalidRegex(String),
    ServerCouldNotParseForm(String),

    MinioCouldNotInitBucket(String, String),
    MinioCouldNotPutObject(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

        let (message, error) = match self {
            Error::DBCouldNotOpenWebSocket(address, error) => (
                format!("Could not connect to database server at: {}", address),
                error,
            ),
            Error::DBAuthenticationFailed(error) => {
                status_code = StatusCode::FORBIDDEN;
                (
                    "Could not authenticate when trying to connect to database server".to_string(),
                    error,
                )
            }
            Error::DBCouldNotCreateTable(name, error) => (
                format!("Could not create table {}", name.to_string()),
                error,
            ),
            Error::DBCouldNotCreateUser(error) => ("Failed to create user.".to_string(), error),
            Error::DBCouldNotConnectToNamespace(name, error) => {
                (format!("Could not connect to namespace: `{}`", name), error)
            }
            Error::DBCouldNotSelectAllUsers(error) => {
                ("Could not query all user".to_string(), error)
            }
            Error::DBCouldNotSelectUser(id, error) => {
                status_code = StatusCode::NOT_FOUND;
                (format!("Could not find user with id: `{}`", id), error)
            }
            Error::DBCouldNotDeleteUser(id, error) => {
                status_code = StatusCode::NOT_FOUND;
                (format!("Could not delete user with id: `{}`", id), error)
            }
            Error::DBCouldNotUpdateUser(id, error) => {
                status_code = StatusCode::NOT_FOUND;
                (format!("Could not update user with id: `{}`", id), error)
            }
            Error::ParseEnvFailedNoSuchFile(file_name, error) => {
                status_code = StatusCode::NOT_FOUND;
                (format!("File not found: `{}`", file_name), error)
            }
            Error::ParseEnvFailedNoSuchKey(key) => {
                status_code = StatusCode::NOT_FOUND;
                (format!("No such key: `{}`", key), String::new())
            }
            Error::ParseEnvFailedWrongFormat(error) => {
                ("Could not parse env file.".to_string(), error)
            }
            Error::ServerNoSuchIP(address, error) => {
                status_code = StatusCode::NOT_FOUND;
                (
                    format!("Could not connect to such address: {}", address),
                    error,
                )
            }
            Error::ServerCouldNotStart(error) => ("Could not start web server".to_string(), error),
            Error::ServerInvalidRegex(error) => {
                ("Could not parse regex correctly".to_string(), error)
            }
            Error::ServerCouldNotParseForm(error) => {
                ("Could not parse form from payload".to_string(), error)
            }
            Error::MinioCouldNotInitBucket(name, error) => {
                (format!("Could not initialize bucket: `{}`", name), error)
            }
            Error::MinioCouldNotPutObject(error) => {
                ("Could not upload object to s3".to_string(), error)
            }
        };
        log::error!("[ERROR]: {}.\n    --> Cause: {}", &message, &error);
        let body = Json(json!({
            "result": {
                "success": false,
                "error": format!("{}", status_code.to_string()),
                "reason": format!("{}", error),
                "message": format!("{}", message),
            },
        }));

        let response = (status_code, body).into_response();
        return response;
    }
}
