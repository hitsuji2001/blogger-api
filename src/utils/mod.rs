pub mod image;
pub mod multipart;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OpChanges<T> {
    op: String,
    path: String,
    value: T,
}
