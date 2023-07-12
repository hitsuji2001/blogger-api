#[derive(Debug)]
pub enum S3Error {
    CouldNotInitBucket,
    CouldNotPutObject,
}
