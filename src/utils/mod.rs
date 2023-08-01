pub mod image;
pub mod multipart;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OpChangesValue {
    Bool(bool),
    Datetime(DateTime<Utc>),
    Id(Thing),
    VecId(Vec<Thing>),
    Text(String),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OpChanges {
    op: String,
    path: String,
    value: Option<OpChangesValue>,
}
