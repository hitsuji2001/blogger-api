#[derive(Debug)]
pub enum ServerError {
    NoSuchIP,
    CouldNotStartServer,
    InvalidRegex,
}
