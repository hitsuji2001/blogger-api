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
    DBCouldNotSelectAllRecords(String),
    DBCouldNotCreateRecord(String),
    DBCouldNotSelectRecord(String, String),
    DBCouldNotDeleteRecord(String, String),
    DBCouldNotUpdateRecord(String, String),
    DBDuplicateUserEmail,

    ParseEnvFailedWrongFormat(String),

    ServerNoSuchIP(String, String),
    ServerCouldNotStart(String),
    ServerCouldNotParseForm(String),
    ServerPermissionDenied(String),
    ServerUnauthorizedUser,
    ServerEmptyFormFromUser,
    ServerUnsupportedMediaType(String),

    MinioCouldNotInitBucket(String, String),
    MinioCouldNotPutObject(String),

    JWTTokenCreationError(String),
    JWTTokenNotFoundOnHeader,
    JWTTokenError(String),
    JWTInvalidAuthHeader,
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
            Error::DBCouldNotCreateTable(name, error) => {
                (format!("Could not create table {}", name), error)
            }
            Error::DBCouldNotCreateRecord(error) => ("Failed to create record.".to_string(), error),
            Error::DBCouldNotConnectToNamespace(name, error) => {
                (format!("Could not connect to namespace: `{}`", name), error)
            }
            Error::DBCouldNotSelectAllRecords(error) => {
                ("Could not query all record".to_string(), error)
            }
            Error::DBCouldNotSelectRecord(id, error) => {
                status_code = StatusCode::NOT_FOUND;
                (format!("Could not find record with id: `{}`", id), error)
            }
            Error::DBCouldNotDeleteRecord(id, error) => {
                status_code = StatusCode::NOT_FOUND;
                (format!("Could not delete record with id: `{}`", id), error)
            }
            Error::DBCouldNotUpdateRecord(id, error) => {
                status_code = StatusCode::NOT_FOUND;
                (format!("Could not update record with id: `{}`", id), error)
            }
            Error::DBDuplicateUserEmail => (
                "Unreachable, there should not be more than one user with the same email"
                    .to_string(),
                "".to_string(),
            ),
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
            Error::ServerPermissionDenied(error) => {
                status_code = StatusCode::FORBIDDEN;
                ("Could not perform action(s)".to_string(), error)
            }
            Error::ServerCouldNotParseForm(error) => {
                ("Could not parse form from payload".to_string(), error)
            }
            Error::ServerUnauthorizedUser => {
                status_code = StatusCode::UNAUTHORIZED;
                ("Unauthorized user".to_string(), "".to_string())
            }
            Error::ServerEmptyFormFromUser => {
                status_code = StatusCode::BAD_REQUEST;
                (
                    "User sent empty form, ignoring update".to_string(),
                    "".to_string(),
                )
            }
            Error::ServerUnsupportedMediaType(media_type) => {
                status_code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
                (
                    format!("Unsupported media type: `{}`", media_type),
                    "".to_string(),
                )
            }
            Error::MinioCouldNotInitBucket(name, error) => {
                (format!("Could not initialize bucket: `{}`", name), error)
            }
            Error::MinioCouldNotPutObject(error) => {
                ("Could not upload object to s3".to_string(), error)
            }
            Error::JWTTokenCreationError(error) => {
                ("Could not create JWT token".to_string(), error)
            }
            Error::JWTTokenNotFoundOnHeader => {
                status_code = StatusCode::BAD_REQUEST;
                (
                    "Could not find JWT token on received headers".to_string(),
                    "".to_string(),
                )
            }
            Error::JWTInvalidAuthHeader => {
                status_code = StatusCode::BAD_REQUEST;
                ("Invalid JWT token".to_string(), "".to_string())
            }
            Error::JWTTokenError(error) => ("Invalid JWT token".to_string(), error),
        };
        log::error!("[ERROR]: {}.\n    --> Cause: {}", &message, &error);
        let body = Json(json!({
            "result": {
                "success": false,
                "error": format!("{}", status_code),
                "reason": format!("{}", error),
                "message": format!("{}", message),
            },
        }));

        (status_code, body).into_response()
    }
}
