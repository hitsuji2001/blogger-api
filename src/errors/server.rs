#[derive(Debug)]
pub enum ServerError {
    NoSuchIP,
    CouldNotStartServer,
    CouldNotParseUserForm,
    InvalidRegex,
}
