use crate::crypto::Key;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug)]
pub struct RecordJson(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Record<'a> {
    pub title: Cow<'a, str>,
    pub lang: Cow<'a, str>,
    pub content: Cow<'a, str>,
    pub saving_time_seconds: u32,
    pub expiration_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveRecordReq<'a> {
    pub title: Cow<'a, str>,
    pub lang: Cow<'a, str>,
    pub content: Cow<'a, str>,
    pub expiration_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveRecordRes {
    pub key: Key,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindRecordRes<'a> {
    pub title: Cow<'a, str>,
    pub lang: Cow<'a, str>,
    pub content: Cow<'a, str>,
    pub saving_time_seconds: u32,
    pub expiration_seconds: u32,
    pub view_count: u64,
}

impl<'a> FindRecordRes<'a> {
    pub fn new(record: Record<'a>, view_count: u64) -> Self {
        Self {
            title: record.title,
            lang: record.lang,
            content: record.content,
            saving_time_seconds: record.saving_time_seconds,
            expiration_seconds: record.expiration_seconds,
            view_count,
        }
    }
}
