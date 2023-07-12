#[derive(Debug)]
pub enum EnvError {
    NoSuchFile,
    NoSuchKey { key: String },
    WrongFormat,
}
