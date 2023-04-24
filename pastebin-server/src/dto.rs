use crate::crypto::Key;
use crate::time::UnixTimestamp;

use serde::{Deserialize, Serialize};

type SharedString = bytestring::ByteString;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub title: SharedString,
    pub lang: SharedString,
    pub content: SharedString,
    pub expiration_seconds: u32,
    pub saving_time: UnixTimestamp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveRecordInput {
    pub title: SharedString,
    pub lang: SharedString,
    pub content: SharedString,
    pub expiration_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveRecordOutput {
    pub key: Key,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FindRecordInput {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FindRecordOutput {
    #[serde(flatten)]
    pub record: Record,
    pub view_count: u64,
}
